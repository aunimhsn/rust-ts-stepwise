//! The errors of the checker: a code with its values and its place, never a sentence, so that
//! the interface writes the message in the language of the learner.

use crate::builtin::Builtin;
use crate::color::Color;
use crate::span::Span;

/// An error of names or of structure in a program whose syntax is right.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckError {
    /// What is wrong.
    pub kind: CheckErrorKind,
    /// The stretch of the source to underline.
    pub span: Span,
}

/// What is wrong in the names or the structure of a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckErrorKind {
    /// A `break` that is in no `while` nor `repeat` of its function or of the main program.
    /// The span is the `break`.
    BreakOutsideLoop,
    /// A `return` that is in no function. The span is its statement.
    ReturnOutsideFunction,
    /// A function defined in a block or in another function: functions are defined at the
    /// top level of the program. The span is its line, up to the `:`.
    NestedFunction,
    /// The name of a new variable, function or parameter that isn't in `snake_case`
    /// (`paintSide`, `côté`, `_sides`). The span is the name.
    NotSnakeCase,
    /// A variable, a function or a parameter that takes the name of a built-in function or of
    /// a color (`let red = 3`, `forward = 3`, `function print():`). The span is the name.
    ReservedName {
        /// What the name belongs to.
        word: ReservedWord,
    },
    /// A parameter that has the name of a parameter before it (`function f(side, side):`).
    /// The span is the parameter.
    DuplicateParameter {
        /// The name.
        name: String,
    },
}

/// A word that belongs to the language and that isn't a keyword: the keywords are refused by
/// the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReservedWord {
    /// A built-in function.
    Builtin(Builtin),
    /// A color.
    Color(Color),
}

impl ReservedWord {
    /// The reserved word written `word`, if it is one.
    #[must_use]
    pub fn from_word(word: &str) -> Option<Self> {
        Builtin::from_word(word)
            .map(Self::Builtin)
            .or_else(|| Color::from_word(word).map(Self::Color))
    }

    /// The word as it is written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Builtin(builtin) => builtin.as_str(),
            Self::Color(color) => color.as_str(),
        }
    }
}
