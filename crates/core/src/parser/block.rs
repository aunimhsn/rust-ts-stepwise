//! The statements that open a block: `if` with its `else if` and its `else`, `while` and
//! `repeat`.
//!
//! The line of such a statement ends with a `:` ([`Parser::colon`]), and its block is the
//! lines under it, between the [`TokenKind::Indent`] and the [`TokenKind::Dedent`] of the
//! lexer ([`Parser::block`]). A block counts for one level of the depth of the tree, as the
//! parentheses do: its statements are parsed by a recursion of the parser.

use super::{ParseError, ParseErrorKind, Parser};
use crate::ast::{Branch, Statement, StatementKind};
use crate::lexer::{Keyword, TokenKind};
use crate::span::{Position, Span};

impl Parser {
    /// An `if` with its `else if` and its `else`, from the `if`.
    pub(super) fn conditional(&mut self) -> Result<Statement, ParseError> {
        let first = self.branch(self.current.span.start)?;
        // The line of the first `if`.
        let span = first.span;
        let mut branches = vec![first];
        let mut otherwise = None;
        while self.current.kind == TokenKind::Keyword(Keyword::Else) {
            let start = self.current.span.start;
            self.advance();
            if self.current.kind == TokenKind::Keyword(Keyword::If) {
                branches.push(self.branch(start)?);
                continue;
            }
            let line = self.colon(start)?;
            otherwise = Some(self.block(line)?);
            if self.current.kind == TokenKind::Keyword(Keyword::Else) {
                return Err(self.error(ParseErrorKind::ElseAfterElse));
            }
            break;
        }
        let kind = StatementKind::If {
            branches,
            otherwise,
        };
        Ok(Statement { kind, span })
    }

    /// `if condition:` with its block, the `if` being the current token and its line
    /// starting at `start`: at the `if`, or at the `else` before it.
    fn branch(&mut self, start: Position) -> Result<Branch, ParseError> {
        self.advance();
        let condition = self.expression()?;
        let span = self.colon(start)?;
        let body = self.block(span)?;
        Ok(Branch {
            condition,
            body,
            span,
        })
    }

    /// `while condition:` with its block, from the `while`.
    pub(super) fn while_loop(&mut self) -> Result<Statement, ParseError> {
        let start = self.current.span.start;
        self.advance();
        let condition = self.expression()?;
        let span = self.colon(start)?;
        let body = self.block(span)?;
        let kind = StatementKind::While { condition, body };
        Ok(Statement { kind, span })
    }

    /// `repeat count:` with its block, from the `repeat`.
    pub(super) fn repeat_loop(&mut self) -> Result<Statement, ParseError> {
        let start = self.current.span.start;
        self.advance();
        let count = self.expression()?;
        let span = self.colon(start)?;
        let body = self.block(span)?;
        let kind = StatementKind::Repeat { count, body };
        Ok(Statement { kind, span })
    }

    /// The `:` that ends a line which opens a block, then the end of that line. Returns the
    /// span of the line, from `start` to its `:`.
    fn colon(&mut self, start: Position) -> Result<Span, ParseError> {
        match self.current.kind {
            TokenKind::Colon => self.advance(),
            TokenKind::RightParen => return Err(self.error(ParseErrorKind::UnmatchedParen)),
            _ => return Err(self.error(ParseErrorKind::ExpectedColon)),
        }
        let span = Span::new(start, self.previous_end);
        match self.current.kind {
            TokenKind::Newline => self.advance(),
            TokenKind::End => {}
            _ => return Err(self.after_colon()),
        }
        Ok(span)
    }

    /// The error of what follows a `:` on its line, from the current token to the end of the
    /// line.
    fn after_colon(&mut self) -> ParseError {
        let mut error = self.error(ParseErrorKind::AfterColon);
        if error.kind == ParseErrorKind::AfterColon {
            while !matches!(self.current.kind, TokenKind::Newline | TokenKind::End) {
                self.advance();
            }
            error.span.end = self.previous_end;
        }
        error
    }

    /// The block under the line `opening`, which ends with a `:`: its statements, from the
    /// first token after that line.
    fn block(&mut self, opening: Span) -> Result<Vec<Statement>, ParseError> {
        if self.current.kind != TokenKind::Indent {
            return Err(self.no_block(opening));
        }
        if !self.fits_deeper() {
            let kind = ParseErrorKind::TooDeep;
            return Err(ParseError {
                kind,
                span: opening,
            });
        }
        self.advance();
        let statements = self.deeper(|parser| {
            let mut statements = Vec::new();
            // The lexer closes every block it opens: the end of the source is never met
            // first.
            while !matches!(parser.current.kind, TokenKind::Dedent | TokenKind::End) {
                statements.push(parser.statement()?);
            }
            Ok(statements)
        })?;
        self.advance();
        Ok(statements)
    }

    /// The error of the line `opening` when no line is indented under it, the current token
    /// being what follows: the block is empty, unless a comment line stands where its lines
    /// would be, indented further than `opening`.
    fn no_block(&self, opening: Span) -> ParseError {
        let next = self.current.span.start;
        let comment = self.comments.iter().find(|comment| {
            let hash = comment.start;
            opening.end < hash && hash < next && hash.column > opening.start.column
        });
        match comment {
            Some(&span) => ParseError {
                kind: ParseErrorKind::CommentOnlyBlock,
                span,
            },
            None => ParseError {
                kind: ParseErrorKind::EmptyBlock,
                span: opening,
            },
        }
    }
}
