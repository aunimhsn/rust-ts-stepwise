//! The expressions: the values, the calls, and the operators with their priorities.
//!
//! [`Parser::operators`] reads a value, then every operator stronger than a floor, each with
//! the value on its right read the same way above the operator's own priority: `1 + 2 * 3`
//! gives `1 + (2 * 3)`, and `1 - 2 - 3` gives `(1 - 2) - 3`.
//!
//! Each method returns the height of the tree it built, so that the depth is checked exactly
//! where a node is made ([`Parser::node`]): a chain of operators deepens the tree without any
//! recursion of the parser. [`Parser::nested`] guards the recursions themselves.

use super::{MAX_DEPTH, ParseError, ParseErrorKind, Parser};
use crate::ast::{BinaryOperator, Expression, ExpressionKind, Name, UnaryOperator};
use crate::lexer::{Keyword, TokenKind};
use crate::span::{Position, Span};

/// The priorities of the operators, from the weakest. A value in front of which `-` is
/// written is read before any of them.
mod priority {
    /// Below every operator: a whole expression.
    pub(super) const LOWEST: u8 = 0;
    pub(super) const OR: u8 = 1;
    pub(super) const AND: u8 = 2;
    /// `not`, written in front of a value: it takes the comparisons that follow, as in
    /// Python.
    pub(super) const NOT: u8 = 3;
    pub(super) const COMPARISON: u8 = 4;
    pub(super) const SUM: u8 = 5;
    pub(super) const PRODUCT: u8 = 6;
}

/// An expression with the height of its tree: 1 for a value alone.
struct Deep {
    expression: Expression,
    height: u32,
}

impl Parser {
    /// A whole expression.
    pub(super) fn expression(&mut self) -> Result<Expression, ParseError> {
        Ok(self.operators(priority::LOWEST)?.expression)
    }

    /// A value, then the operators stronger than `floor` with their values.
    fn operators(&mut self, floor: u8) -> Result<Deep, ParseError> {
        let mut left = self.operand()?;
        // Whether `left` is a comparison written without parentheses.
        let mut compared = false;
        loop {
            if self.current.kind == TokenKind::Equal {
                return Err(self.error(ParseErrorKind::AssignInExpression));
            }
            let Some(operator) = binary_operator(&self.current.kind) else {
                break;
            };
            let priority = priority_of(operator);
            if priority <= floor {
                break;
            }
            if compared && operator.is_comparison() {
                return Err(self.error(ParseErrorKind::ChainedComparison));
            }
            self.advance();
            let right = self.nested(|parser| parser.operators(priority))?;
            let span = Span::new(left.expression.span.start, right.expression.span.end);
            let below = left.height.max(right.height);
            let kind = ExpressionKind::Binary {
                operator,
                left: Box::new(left.expression),
                right: Box::new(right.expression),
            };
            left = self.node(kind, span, below)?;
            compared = operator.is_comparison();
        }
        Ok(left)
    }

    /// A value, with the `-` or the `not` in front of it, its parentheses or its call.
    fn operand(&mut self) -> Result<Deep, ParseError> {
        let span = self.current.span;
        if let Some(name) = self.name()? {
            return self.name_or_call(name);
        }
        let kind = match &mut self.current.kind {
            TokenKind::Number(number) => ExpressionKind::Number(*number),
            TokenKind::Text(text) => ExpressionKind::Text(std::mem::take(text)),
            TokenKind::Keyword(Keyword::True) => ExpressionKind::Boolean(true),
            TokenKind::Keyword(Keyword::False) => ExpressionKind::Boolean(false),
            TokenKind::Minus => return self.unary(UnaryOperator::Negate),
            TokenKind::Keyword(Keyword::Not) => return self.unary(UnaryOperator::Not),
            TokenKind::LeftParen => return self.parenthesized(),
            _ => return Err(self.error(ParseErrorKind::ExpectedValue)),
        };
        self.advance();
        self.node(kind, span, 0)
    }

    /// `-` or `not` in front of a value, from the operator.
    fn unary(&mut self, operator: UnaryOperator) -> Result<Deep, ParseError> {
        let start = self.current.span.start;
        self.advance();
        let operand = match operator {
            // `-` takes the value alone: `-a * b` is `(-a) * b`.
            UnaryOperator::Negate => self.nested(Self::operand)?,
            UnaryOperator::Not => self.nested(|parser| parser.operators(priority::NOT))?,
        };
        let span = Span::new(start, operand.expression.span.end);
        let kind = ExpressionKind::Unary {
            operator,
            operand: Box::new(operand.expression),
        };
        self.node(kind, span, operand.height)
    }

    /// An expression between parentheses, from the `(`. The parentheses leave no node in the
    /// tree, but they widen the span of the expression and count for one level.
    fn parenthesized(&mut self) -> Result<Deep, ParseError> {
        let open = self.current.span;
        self.advance();
        let inner = self.nested(|parser| parser.operators(priority::LOWEST))?;
        let end = self.close(open)?;
        Ok(Deep {
            expression: Expression {
                kind: inner.expression.kind,
                span: Span::new(open.start, end),
            },
            height: inner.height.saturating_add(1),
        })
    }

    /// The name alone, or its call when a `(` follows.
    fn name_or_call(&mut self, name: Name) -> Result<Deep, ParseError> {
        if self.current.kind != TokenKind::LeftParen {
            return self.node(ExpressionKind::Name(name.text), name.span, 0);
        }
        let open = self.current.span;
        self.advance();
        let mut arguments = Vec::new();
        let mut below = 0;
        if self.current.kind != TokenKind::RightParen {
            loop {
                let argument = self.nested(|parser| parser.operators(priority::LOWEST))?;
                below = below.max(argument.height);
                arguments.push(argument.expression);
                if self.current.kind != TokenKind::Comma {
                    break;
                }
                self.advance();
            }
        }
        let end = self.close(open)?;
        let span = Span::new(name.span.start, end);
        let kind = ExpressionKind::Call {
            function: name,
            arguments,
        };
        self.node(kind, span, below)
    }

    /// Takes the `)` that closes the `(` at `open`, and returns the position after it.
    fn close(&mut self, open: Span) -> Result<Position, ParseError> {
        match self.current.kind {
            TokenKind::RightParen => {
                let end = self.current.span.end;
                self.advance();
                Ok(end)
            }
            TokenKind::Newline | TokenKind::End => Err(ParseError {
                kind: ParseErrorKind::UnclosedParen,
                span: open,
            }),
            _ => Err(self.error(ParseErrorKind::UnexpectedToken)),
        }
    }

    /// Parses one level deeper: the value of an operator, the inside of parentheses, a value
    /// given to a call.
    ///
    /// # Errors
    ///
    /// The level is too deep for any value to fit in it.
    fn nested(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<Deep, ParseError>,
    ) -> Result<Deep, ParseError> {
        let depth = self.depth.saturating_add(1);
        if depth >= MAX_DEPTH {
            return Err(self.error(ParseErrorKind::TooDeep));
        }
        self.depth = depth;
        let result = parse(self);
        self.depth = self.depth.saturating_sub(1);
        result
    }

    /// The node `kind` over `span`, above a tree of height `below` (0 for a value alone).
    ///
    /// # Errors
    ///
    /// With the levels around it, the tree is deeper than [`MAX_DEPTH`].
    fn node(&self, kind: ExpressionKind, span: Span, below: u32) -> Result<Deep, ParseError> {
        let height = below.saturating_add(1);
        if self.depth.saturating_add(height) > MAX_DEPTH {
            return Err(ParseError {
                kind: ParseErrorKind::TooDeep,
                span,
            });
        }
        Ok(Deep {
            expression: Expression { kind, span },
            height,
        })
    }
}

/// The operator written between two values that `kind` is, if it is one.
fn binary_operator(kind: &TokenKind) -> Option<BinaryOperator> {
    Some(match kind {
        TokenKind::Plus => BinaryOperator::Add,
        TokenKind::Minus => BinaryOperator::Subtract,
        TokenKind::Star => BinaryOperator::Multiply,
        TokenKind::Slash => BinaryOperator::Divide,
        TokenKind::Percent => BinaryOperator::Remainder,
        TokenKind::EqualEqual => BinaryOperator::Equal,
        TokenKind::NotEqual => BinaryOperator::NotEqual,
        TokenKind::Less => BinaryOperator::Less,
        TokenKind::LessEqual => BinaryOperator::LessEqual,
        TokenKind::Greater => BinaryOperator::Greater,
        TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
        TokenKind::Keyword(Keyword::And) => BinaryOperator::And,
        TokenKind::Keyword(Keyword::Or) => BinaryOperator::Or,
        _ => return None,
    })
}

const fn priority_of(operator: BinaryOperator) -> u8 {
    match operator {
        BinaryOperator::Or => priority::OR,
        BinaryOperator::And => priority::AND,
        BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::Less
        | BinaryOperator::LessEqual
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEqual => priority::COMPARISON,
        BinaryOperator::Add | BinaryOperator::Subtract => priority::SUM,
        BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Remainder => {
            priority::PRODUCT
        }
    }
}
