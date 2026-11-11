//! The tokens the lexer gives to the parser.

use crate::span::Span;
use crate::word_enum::word_enum;

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

word_enum! {
    /// The keywords of the language, reserved for it.
    ///
    /// The words a learner may bring from another language (`elif`, `def`…) are names: the
    /// parser and the checker give them their own messages.
    pub enum Keyword {
        /// `let`
        Let => "let",
        /// `if`
        If => "if",
        /// `else`
        Else => "else",
        /// `while`
        While => "while",
        /// `repeat`
        Repeat => "repeat",
        /// `break`
        Break => "break",
        /// `function`
        Function => "function",
        /// `return`
        Return => "return",
        /// `and`
        And => "and",
        /// `or`
        Or => "or",
        /// `not`
        Not => "not",
        /// `true`
        True => "true",
        /// `false`
        False => "false",
    }
}
