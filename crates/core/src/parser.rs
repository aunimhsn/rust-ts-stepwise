//! The parser: turns the tokens of a program into its syntax tree.
//!
//! Written by hand, as a recursive descent: one method per statement, and for the
//! expressions a loop over the operators that climbs their priorities (see `expression.rs`).
//! It stops at the first error, since what follows can't be read reliably, and leaves to
//! the checker what needs the names of the program: a function defined in a block, a `break`
//! outside a loop, an expression alone on its line that isn't a call.
//!
//! The words of other languages (`elif`, `def`…) are names for the lexer. The parser refuses
//! them wherever it meets one, in [`Parser::name`] where a name is right and in
//! [`Parser::error`] where it isn't: whatever the error would have been, the word explains it
//! better.

mod error;
mod expression;

#[cfg(test)]
mod tests;

pub use error::{ForeignWord, ParseError, ParseErrorKind, SyntaxError};

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{Name, Program, Statement, StatementKind};
use crate::lexer::{Keyword, Token, TokenKind, tokenize};
use crate::span::{Position, Span};

/// The deepest a syntax tree goes: the parentheses, the operators and the calls around a
/// value each count for one level, and the value itself for one.
///
/// A long chain of operators counts too (`1 + 1 + 1 + …` is a tree as deep as it is long).
/// The limit keeps the recursions of the parser, the checker and the interpreter, and the one
/// that frees the tree, far from the end of the stack of the browser.
pub const MAX_DEPTH: u32 = 200;

/// The syntax tree of `source`.
///
/// # Errors
///
/// The first syntax error of the source, from the lexer or from the parser.
pub fn parse(source: &str) -> Result<Program, SyntaxError> {
    let tokens = tokenize(source)?;
    Ok(Parser::new(tokens).program()?)
}

/// The place of the parser in the tokens.
struct Parser {
    /// The token to read next. It stays [`TokenKind::End`] once the end is reached.
    current: Token,
    /// The tokens after it.
    rest: Peekable<IntoIter<Token>>,
    /// The end of the last token read: the end of the statement being parsed.
    previous_end: Position,
    /// The number of levels of the tree around what is being parsed.
    depth: u32,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let start = Position::new(1, 1);
        let mut rest = tokens.into_iter().peekable();
        let current = rest.next().unwrap_or(Token {
            kind: TokenKind::End,
            span: Span::at(start),
        });
        Self {
            current,
            rest,
            previous_end: start,
            depth: 0,
        }
    }

    /// Moves to the next token.
    fn advance(&mut self) {
        self.previous_end = self.current.span.end;
        if let Some(next) = self.rest.next() {
            self.current = next;
        }
    }

    /// The error `kind` at the current token, or the error of a word of another language
    /// when the token is one.
    fn error(&self, kind: ParseErrorKind) -> ParseError {
        let foreign = match &self.current.kind {
            TokenKind::Name(text) => ForeignWord::from_word(text),
            _ => None,
        };
        ParseError {
            kind: foreign.map_or(kind, |word| ParseErrorKind::ForeignWord { word }),
            span: self.current.span,
        }
    }

    /// Takes the current token if it is a name.
    ///
    /// # Errors
    ///
    /// The name is a word of another language.
    fn name(&mut self) -> Result<Option<Name>, ParseError> {
        let span = self.current.span;
        let TokenKind::Name(text) = &mut self.current.kind else {
            return Ok(None);
        };
        if let Some(word) = ForeignWord::from_word(text) {
            let kind = ParseErrorKind::ForeignWord { word };
            return Err(ParseError { kind, span });
        }
        let text = std::mem::take(text);
        self.advance();
        Ok(Some(Name { text, span }))
    }

    fn program(mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while self.current.kind != TokenKind::End {
            statements.push(self.statement()?);
        }
        Ok(Program { statements })
    }

    /// The statement of one line, with the end of the line.
    fn statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current.span.start;
        let assigns = self.next_is_equal();
        let kind = match self.current.kind {
            TokenKind::Indent => return Err(self.error(ParseErrorKind::UnexpectedIndent)),
            TokenKind::Keyword(Keyword::Let) => self.declaration()?,
            TokenKind::Keyword(keyword) if assigns => {
                return Err(self.error(ParseErrorKind::KeywordAsName { keyword }));
            }
            TokenKind::Keyword(Keyword::Break) => {
                self.advance();
                StatementKind::Break
            }
            TokenKind::Keyword(Keyword::Return) => {
                self.advance();
                let value = match self.current.kind {
                    TokenKind::Newline | TokenKind::End => None,
                    _ => Some(self.expression()?),
                };
                StatementKind::Return { value }
            }
            // The statements that open a block come with step 6b.
            TokenKind::Keyword(
                Keyword::If | Keyword::Else | Keyword::While | Keyword::Repeat | Keyword::Function,
            ) => return Err(self.error(ParseErrorKind::UnexpectedToken)),
            TokenKind::Name(_) if assigns => self.assignment()?,
            _ => StatementKind::Expression(self.expression()?),
        };
        let span = Span::new(start, self.previous_end);
        self.end_of_line()?;
        Ok(Statement { kind, span })
    }

    /// Whether the token after the current one is a `=`.
    fn next_is_equal(&mut self) -> bool {
        self.rest
            .peek()
            .is_some_and(|token| token.kind == TokenKind::Equal)
    }

    /// `let name = value`, from its `let`.
    fn declaration(&mut self) -> Result<StatementKind, ParseError> {
        self.advance();
        let Some(name) = self.name()? else {
            return Err(self.error(match self.current.kind {
                TokenKind::Keyword(keyword) => ParseErrorKind::KeywordAsName { keyword },
                _ => ParseErrorKind::ExpectedName,
            }));
        };
        if self.current.kind != TokenKind::Equal {
            return Err(self.error(ParseErrorKind::ExpectedEqual));
        }
        self.advance();
        let value = self.expression()?;
        Ok(StatementKind::Let { name, value })
    }

    /// `name = value`, from its name.
    fn assignment(&mut self) -> Result<StatementKind, ParseError> {
        let Some(name) = self.name()? else {
            return Err(self.error(ParseErrorKind::ExpectedName));
        };
        self.advance();
        let value = self.expression()?;
        Ok(StatementKind::Assign { name, value })
    }

    /// The end of the line, after a whole statement.
    fn end_of_line(&mut self) -> Result<(), ParseError> {
        match self.current.kind {
            TokenKind::Newline => {
                self.advance();
                Ok(())
            }
            TokenKind::End => Ok(()),
            TokenKind::RightParen => Err(self.error(ParseErrorKind::UnmatchedParen)),
            _ => Err(self.error(ParseErrorKind::UnexpectedToken)),
        }
    }
}
