//! The lexer: turns the source of a program into the tokens the parser reads.
//!
//! It reads the source line by line, since a statement never spans two lines (see *Lines and
//! indentation* in `specs.md`):
//!
//! - an empty line or a comment line is skipped, whatever its indentation, tabs and other
//!   blank characters included; the place of each comment line is kept aside, for the parser
//!   to tell a block that holds only comments from an empty one;
//! - the indentation of any other line, in spaces only, opens a block with
//!   [`TokenKind::Indent`] when it is four spaces more than the line above, or closes blocks
//!   with one [`TokenKind::Dedent`] each when it goes back to an enclosing line; any other
//!   indentation is an error;
//! - the tokens of the line follow, up to its end or a `#`, then a [`TokenKind::Newline`].
//!
//! The source ends with a [`TokenKind::Dedent`] per block still open, then [`TokenKind::End`].
//!
//! Whether an indented line follows a `:`, and whether a `:` is followed by an indented line,
//! is left to the parser: it can then say that the `:` is missing on the line above, rather
//! than that the next line is indented. The lexer stops at the first error, as the parser
//! does, since what follows can't be read reliably.

mod error;
mod token;

#[cfg(test)]
mod tests;

pub use error::{LexError, LexErrorKind};
pub use token::{Keyword, Token, TokenKind};

use std::iter::Peekable;
use std::str::Chars;

use crate::span::{Position, Span};

/// The number of spaces that indent a block further than the line that opens it.
pub const INDENT_WIDTH: u32 = 4;

/// What the lexer reads in a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexed {
    /// The tokens, ending with [`TokenKind::End`].
    pub tokens: Vec<Token>,
    /// The comment of each line that holds only a comment, from its `#` to the end of the
    /// line, in the order of the source. These lines give no token.
    pub comments: Vec<Span>,
}

/// The tokens of `source`, with the places of its comment lines.
///
/// A line break is `\n` or `\r\n`.
///
/// # Errors
///
/// The first error of the source: a tab, a wrong indentation, a text not closed or with an
/// unknown escape, a number too large, a word that starts with a digit, or a character that
/// has no meaning outside a text or a comment.
pub fn tokenize(source: &str) -> Result<Lexed, LexError> {
    let mut lexer = Lexer {
        tokens: Vec::new(),
        comments: Vec::new(),
        levels: Vec::new(),
    };
    let mut end = Position::new(1, 1);
    for (index, line) in source.split('\n').enumerate() {
        let number = u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1);
        end = lexer.line(line.strip_suffix('\r').unwrap_or(line), number)?;
    }
    for _ in 0..lexer.levels.len() {
        lexer.push(TokenKind::Dedent, Span::at(end));
    }
    lexer.push(TokenKind::End, Span::at(end));
    Ok(Lexed {
        tokens: lexer.tokens,
        comments: lexer.comments,
    })
}

/// The state kept from one line to the next.
struct Lexer {
    tokens: Vec<Token>,
    /// The comment of each comment line read, from its `#` to the end of the line.
    comments: Vec<Span>,
    /// The indentation of each block still open, in spaces, the innermost last: empty at the
    /// top level.
    levels: Vec<u32>,
}

impl Lexer {
    fn push(&mut self, kind: TokenKind, span: Span) {
        self.tokens.push(Token { kind, span });
    }

    /// The indentation of the innermost block open, `0` at the top level.
    fn level(&self) -> u32 {
        self.levels.last().copied().unwrap_or(0)
    }

    /// Reads one line, without its line break, and returns the position of its end.
    fn line(&mut self, line: &str, number: u32) -> Result<Position, LexError> {
        let length = u32::try_from(line.chars().count()).unwrap_or(u32::MAX);
        let end = Position::new(number, length.saturating_add(1));
        // Blank characters of any kind, tabs and non-breaking spaces included: a line that
        // looks empty is empty.
        let content = line.trim_start();
        if content.is_empty() {
            return Ok(end);
        }
        if content.starts_with('#') {
            let before = line.chars().count().saturating_sub(content.chars().count());
            let column = u32::try_from(before).unwrap_or(u32::MAX).saturating_add(1);
            self.comments
                .push(Span::new(Position::new(number, column), end));
            return Ok(end);
        }
        let mut cursor = Cursor::new(line, number);

        let mut spaces: u32 = 0;
        let mut tab = None;
        while let Some(character @ (' ' | '\t')) = cursor.peek() {
            if character == ' ' {
                spaces = spaces.saturating_add(1);
            } else if tab.is_none() {
                tab = Some(cursor.position());
            }
            cursor.bump();
        }
        let first = cursor.position();
        if let Some(tab) = tab {
            return Err(LexError {
                kind: LexErrorKind::Tab,
                span: Span::new(tab, Position::new(number, tab.column.saturating_add(1))),
            });
        }
        self.indent(spaces, Span::new(Position::new(number, 1), first))?;

        let mut last_end = first;
        while let Some(token) = cursor.token()? {
            last_end = token.span.end;
            self.tokens.push(token);
        }
        self.push(TokenKind::Newline, Span::at(last_end));
        Ok(end)
    }

    /// Opens or closes the blocks for a line indented by `found` spaces, `span` covering
    /// its indentation.
    fn indent(&mut self, found: u32, span: Span) -> Result<(), LexError> {
        let current = self.level();
        if found > current {
            let above = current.saturating_add(INDENT_WIDTH);
            if found != above {
                return Err(LexError {
                    kind: LexErrorKind::WrongIndent {
                        found,
                        below: current,
                        above,
                    },
                    span,
                });
            }
            self.levels.push(found);
            self.push(TokenKind::Indent, span);
            return Ok(());
        }
        while let Some(&above) = self.levels.last()
            && found < above
        {
            self.levels.pop();
            let below = self.level();
            if found > below {
                return Err(LexError {
                    kind: LexErrorKind::WrongIndent {
                        found,
                        below,
                        above,
                    },
                    span,
                });
            }
            self.push(TokenKind::Dedent, Span::at(span.end));
        }
        Ok(())
    }
}

/// Reads the characters of one line, keeping their position.
struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    line: u32,
    /// The column of the next character.
    column: u32,
}

impl<'a> Cursor<'a> {
    fn new(line: &'a str, number: u32) -> Self {
        Self {
            chars: line.chars().peekable(),
            line: number,
            column: 1,
        }
    }

    /// The position of the next character.
    fn position(&self) -> Position {
        Position::new(self.line, self.column)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Takes the next character if `accept` accepts it: the only place that moves the column.
    fn bump_if(&mut self, accept: impl FnOnce(char) -> bool) -> Option<char> {
        let character = self.chars.next_if(|&character| accept(character))?;
        self.column = self.column.saturating_add(1);
        Some(character)
    }

    /// Takes the next character.
    fn bump(&mut self) -> Option<char> {
        self.bump_if(|_| true)
    }

    /// Takes the next character if it is `expected`.
    fn eat(&mut self, expected: char) -> bool {
        self.bump_if(|character| character == expected).is_some()
    }

    /// The error `kind` over the characters read since `start`.
    fn error(&self, kind: LexErrorKind, start: Position) -> LexError {
        LexError {
            kind,
            span: Span::new(start, self.position()),
        }
    }

    /// The next token of the line, or `None` at its end or at a comment.
    fn token(&mut self) -> Result<Option<Token>, LexError> {
        while self.eat(' ') {}
        let start = self.position();
        let Some(character) = self.bump() else {
            return Ok(None);
        };
        let kind = match character {
            '#' => return Ok(None),
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '=' if self.eat('=') => TokenKind::EqualEqual,
            '=' => TokenKind::Equal,
            '!' if self.eat('=') => TokenKind::NotEqual,
            '<' if self.eat('=') => TokenKind::LessEqual,
            '<' => TokenKind::Less,
            '>' if self.eat('=') => TokenKind::GreaterEqual,
            '>' => TokenKind::Greater,
            '"' => self.text(start)?,
            '0'..='9' => self.number(character, start)?,
            _ if is_word(character) => keyword_or_name(self.word(character)),
            _ => {
                let kind = self.unexpected(character);
                return Err(self.error(kind, start));
            }
        };
        Ok(Some(Token {
            kind,
            span: Span::new(start, self.position()),
        }))
    }

    /// The error for `character`, just read, which starts no token. A symbol of another
    /// language reads its second character, when it has one (`&&`, `||`).
    fn unexpected(&mut self, character: char) -> LexErrorKind {
        let symbol_for = |symbol, keyword| LexErrorKind::SymbolForKeyword { symbol, keyword };
        match character {
            '\t' => LexErrorKind::Tab,
            '\'' => LexErrorKind::SingleQuote,
            '!' => symbol_for("!", Keyword::Not),
            '&' if self.eat('&') => symbol_for("&&", Keyword::And),
            '&' => symbol_for("&", Keyword::And),
            '|' if self.eat('|') => symbol_for("||", Keyword::Or),
            '|' => symbol_for("|", Keyword::Or),
            _ => LexErrorKind::UnexpectedCharacter { character },
        }
    }

    /// The word that starts with `first`: letters, digits and underscores.
    fn word(&mut self, first: char) -> String {
        let mut word = String::from(first);
        while let Some(character) = self.bump_if(is_word) {
            word.push(character);
        }
        word
    }

    /// The number that starts with the digit `first`, from `start`.
    fn number(&mut self, first: char, start: Position) -> Result<TokenKind, LexError> {
        let word = self.word(first);
        if !word.chars().all(|character| character.is_ascii_digit()) {
            return Err(self.error(LexErrorKind::NameStartsWithDigit { word }, start));
        }
        word.parse()
            .map(TokenKind::Number)
            .map_err(|_| self.error(LexErrorKind::NumberTooLarge, start))
    }

    /// The text whose opening `"`, already read, is at `start`.
    fn text(&mut self, start: Position) -> Result<TokenKind, LexError> {
        let mut text = String::new();
        loop {
            let escape = self.position();
            match self.bump() {
                Some('"') => return Ok(TokenKind::Text(text)),
                Some('\\') => match self.bump() {
                    Some('"') => text.push('"'),
                    Some('\\') => text.push('\\'),
                    Some('n') => text.push('\n'),
                    Some(character) => {
                        return Err(
                            self.error(LexErrorKind::UnknownEscape { escape: character }, escape)
                        );
                    }
                    None => return Err(self.error(LexErrorKind::UnclosedText, start)),
                },
                Some(character) => text.push(character),
                None => return Err(self.error(LexErrorKind::UnclosedText, start)),
            }
        }
    }
}

/// Whether `character` can be part of a name: a letter, a digit or an underscore, in any
/// alphabet, so that the checker can tell the learner how to write `côté` or `paintSide`.
fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// The keyword or the name `word`.
fn keyword_or_name(word: String) -> TokenKind {
    Keyword::from_word(&word).map_or(TokenKind::Name(word), TokenKind::Keyword)
}
