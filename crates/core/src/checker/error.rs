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
    /// A name read that is no variable of the program (`sides = sidse + 1`). The span is the
    /// name.
    UnknownName {
        /// The name.
        name: String,
    },
    /// A name given a value by `=` alone that is no variable of the program: a variable is
    /// declared with `let` (`sidse = sides + 1`). The span is the name.
    UndeclaredVariableChanged {
        /// The name.
        name: String,
    },
    /// A variable read or changed above its `let`, which is further down in the block or in
    /// a block around it: a variable exists from its `let`. The span is the name.
    VariableBeforeLet {
        /// The name.
        name: String,
        /// The name in its `let`.
        declared: Span,
    },
    /// A variable read or changed after the end of the block where it is declared, the body
    /// of a function being a block: a variable exists until the end of its block. The span is
    /// the name.
    VariableAfterBlock {
        /// The name.
        name: String,
        /// The name in its `let`: the closest above, when several blocks declared it.
        declared: Span,
    },
    /// A variable of the main program read or changed in a function, wherever its `let` is:
    /// a function sees only its parameters and its own variables. The span is the name.
    MainVariableInFunction {
        /// The name.
        name: String,
    },
    /// A `let` of a name that is already a variable there, declared in the block or in a
    /// block around it, or a parameter of the function: `let` declares once, `=` alone
    /// changes the value. The span is the name.
    AlreadyDeclared {
        /// The name.
        name: String,
        /// The name where it was declared first.
        first: Span,
    },
    /// A function that has the name of a function defined above it: a name is one thing in
    /// the whole program. The first stays the function. The span is the name.
    DuplicateFunction {
        /// The name.
        name: String,
        /// The name in the first definition.
        first: Span,
    },
    /// A variable or a parameter that takes the name of a function of the program, wherever
    /// that function is defined, in a `let`, with `=` alone or between the parentheses of a
    /// function (`let paint_side = 3`, `paint_side = 3`). The span is the name.
    NameOfFunction {
        /// The name.
        name: String,
        /// The name in the definition of the function.
        defined: Span,
    },
    /// A function of the program called above its definition, in the reading order of the
    /// source, even from the body of a function that runs later: a function is defined above
    /// the lines that call it. The span is the name in the call.
    CalledBeforeDefinition {
        /// The name.
        name: String,
        /// The name in the definition of the function.
        defined: Span,
    },
    /// A call that doesn't give as many values as the function has parameters, built-in
    /// functions included (`forward(3)`). The span is the name in the call, as for every error
    /// of a call.
    WrongArgumentCount {
        /// The name of the function.
        name: String,
        /// The number of parameters of the function.
        expected: usize,
        /// The number of values the call gives.
        given: usize,
    },
    /// A function, built-in or of the program, named without its parentheses (`forward`,
    /// `let go = forward`). The span is the name, with the parentheses around it if any.
    FunctionWithoutParentheses {
        /// The name.
        name: String,
    },
    /// A variable that exists at this place called like a function (`sides()`): a variable
    /// isn't called. The span is the name.
    VariableCalled {
        /// The name.
        name: String,
    },
    /// A call of a name that is no function, nor a variable of this place (`paint_sid()`,
    /// `red()`). The span is the name.
    UnknownFunction {
        /// The name.
        name: String,
    },
    /// An expression alone on its line that isn't a call (`3 + 4`, `sides`, `sides == 4`):
    /// it does nothing. A function named without its parentheses has its own error only.
    /// The span is the statement.
    ExpressionAlone,
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

    /// Whether `word` is a reserved word.
    #[must_use]
    pub fn is_reserved(word: &str) -> bool {
        Self::from_word(word).is_some()
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
