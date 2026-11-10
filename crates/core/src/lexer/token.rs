//! The tokens the lexer gives to the parser.

use crate::span::Span;

/// A token, with the stretch of the source it comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// What the token is.
    pub kind: TokenKind,
    /// Where it is in the source.
    pub span: Span,
}

/// What a token is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// A name of a variable, a function, a parameter, a built-in function or a color, as
    /// written: the checker refuses the names that aren't in `snake_case` or are reserved.
    Name(String),
    /// One of the keywords of the language.
    Keyword(Keyword),
    /// A whole number, written without its sign: `-7` is [`TokenKind::Minus`] then `7`.
    Number(i64),
    /// A text, its escapes already replaced: `"a\"b"` gives `a"b`.
    Text(String),
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `,`
    Comma,
    /// `:`, at the end of a line that opens a block.
    Colon,
    /// `=`, which declares or changes a variable.
    Equal,
    /// `==`
    EqualEqual,
    /// `!=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// The end of a line that holds a statement. Its span is empty, just after the last token
    /// of the line, so that an error such as a missing `:` points where the `:` belongs.
    Newline,
    /// The start of a block: the line is indented by four more spaces than the line above.
    /// Its span covers the indentation.
    Indent,
    /// The end of a block: the line goes back to the indentation of an enclosing line. A line
    /// that closes several blocks gives one per block. Its span is empty, before the first
    /// token of the line (or at the end of the source for the blocks still open there).
    Dedent,
    /// The end of the source, always the last token. Its span is empty.
    End,
}

/// The keywords of the language, reserved for it.
///
/// The words a learner may bring from another language (`elif`, `def`…) are names: the
/// parser and the checker give them their own messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    /// `let`
    Let,
    /// `if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `repeat`
    Repeat,
    /// `break`
    Break,
    /// `function`
    Function,
    /// `return`
    Return,
    /// `and`
    And,
    /// `or`
    Or,
    /// `not`
    Not,
    /// `true`
    True,
    /// `false`
    False,
}

impl Keyword {
    /// Every keyword, in the order of the specifications.
    pub const ALL: [Self; 13] = [
        Self::Let,
        Self::If,
        Self::Else,
        Self::While,
        Self::Repeat,
        Self::Break,
        Self::Function,
        Self::Return,
        Self::And,
        Self::Or,
        Self::Not,
        Self::True,
        Self::False,
    ];

    /// The keyword written `word`, if it is one.
    #[must_use]
    pub fn from_word(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|keyword| keyword.as_str() == word)
    }

    /// The keyword as it is written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Let => "let",
            Self::If => "if",
            Self::Else => "else",
            Self::While => "while",
            Self::Repeat => "repeat",
            Self::Break => "break",
            Self::Function => "function",
            Self::Return => "return",
            Self::And => "and",
            Self::Or => "or",
            Self::Not => "not",
            Self::True => "true",
            Self::False => "false",
        }
    }
}
