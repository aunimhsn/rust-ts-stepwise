//! The errors of the parser: a code with its values and its place, never a sentence, so that
//! the interface writes the message in the language of the learner.

use crate::lexer::{Keyword, LexError};
use crate::span::Span;

/// The first syntax error of a program, found by the lexer or by the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    /// An error in the characters of the source.
    Lex(LexError),
    /// An error in the order of its words.
    Parse(ParseError),
}

impl SyntaxError {
    /// The stretch of the source to underline.
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Lex(error) => error.span,
            Self::Parse(error) => error.span,
        }
    }
}

impl From<LexError> for SyntaxError {
    fn from(error: LexError) -> Self {
        Self::Lex(error)
    }
}

impl From<ParseError> for SyntaxError {
    fn from(error: ParseError) -> Self {
        Self::Parse(error)
    }
}

/// The first error found in the tokens, which stops the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// What is wrong.
    pub kind: ParseErrorKind,
    /// The stretch of the source to underline. It is empty at the end of a line, when
    /// something is missing there.
    pub span: Span,
}

/// What is wrong in the order of the tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// A value is missing where one is expected: after an operator, a `(`, a `,` or a `=`
    /// (`sides +`, `print(, 1)`, `let sides =`). The span is what stands there instead, or
    /// the end of the line.
    ExpectedValue,
    /// A `,` after the last value of a call (`paint_side(3,)`). The span is the `,`.
    TrailingComma,
    /// A `not` right after an operator stronger than it (`a == not b`, `-not a`): it is
    /// written at the start of an expression, after `and` or `or`, or between parentheses.
    NotAfterOperator,
    /// What follows can't be there: the statement or the value before it is complete
    /// (`forward() forward()`, `print("a" "b")`, `break 3`).
    UnexpectedToken,
    /// A `(` whose `)` is missing on its line, the line or its `:` ending where a value or
    /// the `)` is expected (`forward(`, `if front_is_clear(:`). The span is the `(`.
    UnclosedParen,
    /// A `)` that closes no `(`.
    UnmatchedParen,
    /// A `=` inside an expression, where `==` compares two values (`if x = 3:`).
    AssignInExpression,
    /// A comparison that follows another without parentheses (`1 < x < 5`): two comparisons
    /// are joined with `and`. The span is the second operator.
    ChainedComparison,
    /// A word of another language, wherever it is written.
    ForeignWord {
        /// The word.
        word: ForeignWord,
    },
    /// No name after `let`.
    ExpectedName,
    /// A keyword where a name is expected (`let repeat = 3`, `repeat = 3`): keywords are
    /// reserved.
    KeywordAsName {
        /// The keyword.
        keyword: Keyword,
    },
    /// No `=` after the name of a `let`: a variable is declared with its first value.
    ExpectedEqual,
    /// A line indented further than the line above, which opens no block. The span is its
    /// indentation.
    UnexpectedIndent,
    /// No `:` at the end of a line that opens a block (`if front_is_clear()`). The span is
    /// what stands there instead, or the end of the line.
    ExpectedColon,
    /// Something after the `:` of a line that opens a block (`if front_is_clear(): forward()`):
    /// the block goes on the lines under it. The span is what follows the `:`, up to the end
    /// of the line.
    AfterColon,
    /// No line under a line that opens a block. The span is that line.
    EmptyBlock,
    /// Only comments under a line that opens a block, at least one of them indented further
    /// than that line: the comments don't count as lines. The span is the first of these
    /// comments.
    CommentOnlyBlock,
    /// An `else` that follows no block of an `if` at its indentation. The span is the `else`.
    ElseWithoutIf,
    /// An `else` or an `else if` after the `else` of an `if`: the `else` comes last, once.
    /// The span is the `else` too many.
    ElseAfterElse,
    /// Parentheses, operators, calls and blocks nested deeper than
    /// [`MAX_DEPTH`](super::MAX_DEPTH). For a block, the span is the line that opens it.
    TooDeep,
}

/// The words a learner may write from another language, reserved so that each gets a message
/// that shows the Stepwise way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignWord {
    /// `elif`, written `else if`.
    Elif,
    /// `def`, written `function`.
    Def,
    /// `var`, written `let`.
    Var,
    /// `const`, written `let`.
    Const,
    /// `for`
    For,
    /// `continue`
    Continue,
    /// `pass`
    Pass,
    /// `none`
    NoneWord,
    /// `null`
    Null,
}

impl ForeignWord {
    /// Every word, in the order of the specifications.
    pub const ALL: [Self; 9] = [
        Self::Elif,
        Self::Def,
        Self::Var,
        Self::Const,
        Self::For,
        Self::Continue,
        Self::Pass,
        Self::NoneWord,
        Self::Null,
    ];

    /// The word written `word`, if it is one of them.
    #[must_use]
    pub fn from_word(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|foreign| foreign.as_str() == word)
    }

    /// The word as it is written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elif => "elif",
            Self::Def => "def",
            Self::Var => "var",
            Self::Const => "const",
            Self::For => "for",
            Self::Continue => "continue",
            Self::Pass => "pass",
            Self::NoneWord => "none",
            Self::Null => "null",
        }
    }
}
