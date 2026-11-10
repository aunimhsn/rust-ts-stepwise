//! The statements that open a block: `if` with its `else if` and its `else`, `while` and
//! `repeat`.
//!
//! The line of such a statement ends with a `:` ([`Parser::colon`]), and its block is the
//! lines under it, between the [`TokenKind::Indent`] and the [`TokenKind::Dedent`] of the
//! lexer ([`Parser::block`]). A block counts for one level of the depth of the tree, as the
//! parentheses do: its statements are parsed by a recursion of the parser. The blocks
//! themselves nest up to [`MAX_BLOCK_DEPTH`].

use super::{MAX_BLOCK_DEPTH, ParseError, ParseErrorKind, Parser};
use crate::ast::{Branch, Statement, StatementKind};
use crate::lexer::{Keyword, TokenKind};
use crate::span::{Position, Span};

impl Parser {
    /// An `if` with its `else if` and its `else`, from the `if`.
    pub(super) fn conditional(&mut self) -> Result<Statement, ParseError> {
        let first = self.opening(self.current.span.start)?;
        // The line of the first `if`.
        let span = first.span;
        let mut branches = vec![first];
        let mut otherwise = None;
        while self.current.kind == TokenKind::Keyword(Keyword::Else) {
            let start = self.current.span.start;
            self.not_assigned(Keyword::Else)?;
            self.advance();
            if self.current.kind != TokenKind::Keyword(Keyword::If) {
                let line = self.colon(start)?;
                otherwise = Some(self.block(line)?);
                if self.current.kind == TokenKind::Keyword(Keyword::Else) {
                    return Err(self.error(ParseErrorKind::ElseAfterElse));
                }
                break;
            }
            self.not_assigned(Keyword::If)?;
            branches.push(self.opening(start)?);
        }
        let kind = StatementKind::If {
            branches,
            otherwise,
        };
        Ok(Statement { kind, span })
    }

    /// `while condition:` with its block, from the `while`.
    pub(super) fn while_loop(&mut self) -> Result<Statement, ParseError> {
        let Branch {
            condition,
            body,
            span,
        } = self.opening(self.current.span.start)?;
        let kind = StatementKind::While { condition, body };
        Ok(Statement { kind, span })
    }

    /// `repeat count:` with its block, from the `repeat`.
    pub(super) fn repeat_loop(&mut self) -> Result<Statement, ParseError> {
        let Branch {
            condition: count,
            body,
            span,
        } = self.opening(self.current.span.start)?;
        let kind = StatementKind::Repeat { count, body };
        Ok(Statement { kind, span })
    }

    /// A keyword, the current token, then an expression, a `:` and a block: what `if`,
    /// `while` and `repeat` share. The line starts at `start`: at the keyword, or at the
    /// `else` before an `if`.
    fn opening(&mut self, start: Position) -> Result<Branch, ParseError> {
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

    /// The error of `keyword`, the current token, when a `=` follows it: the same as at the
    /// start of a statement, for the `else` and the `if` that come after a block.
    fn not_assigned(&mut self, keyword: Keyword) -> Result<(), ParseError> {
        if self.next_is_equal() {
            return Err(self.error(ParseErrorKind::KeywordAsName { keyword }));
        }
        Ok(())
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
        // Around a statement, the levels are all blocks.
        if self.depth >= MAX_BLOCK_DEPTH {
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
    ///
    /// The columns count characters: a comment indented by tabs under a line indented by
    /// spaces may be taken for one that isn't in the block. Both errors say that the block
    /// has no line, and the editor writes spaces.
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
