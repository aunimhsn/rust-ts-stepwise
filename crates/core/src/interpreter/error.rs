//! The errors of a program while it runs: a code with its values and its place, never a
//! sentence, so that the interface writes the message in the language of the learner.

use super::value::ValueType;
use crate::ast::{BinaryOperator, UnaryOperator};
use crate::span::Span;

/// An error that stops a program while it runs: only the values decide it, so the checks
/// before running can't find it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
    /// What went wrong.
    pub kind: RuntimeErrorKind,
    /// The stretch of the source to show.
    pub span: Span,
}

/// What stops a program while it runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeErrorKind {
    /// The condition of an `if`, of an `else if` or of a `while` that isn't a boolean
    /// (`if 3:`): there is no "truthy" value. The span is the condition.
    ConditionNotBoolean {
        /// The type of its value.
        found: ValueType,
    },
    /// The count of a `repeat` that isn't a whole number (`repeat "twice":`). The span is the
    /// count.
    RepeatCountNotNumber {
        /// The type of its value.
        found: ValueType,
    },
    /// The count of a `repeat` below zero. The span is the count.
    NegativeRepeatCount {
        /// Its value.
        count: i64,
    },
    /// `-` in front of a value that isn't a whole number, or `not` in front of one that isn't
    /// a boolean (`-"a"`, `not 3`). The span is the operator with its value.
    UnaryOperand {
        /// The operator.
        operator: UnaryOperator,
        /// The type of the value it is in front of.
        found: ValueType,
    },
    /// An operator between two values it doesn't take: an arithmetic or an order on values
    /// that aren't whole numbers, `+` between a text and a number, `==` or `!=` between two
    /// values of different types (`"Sides: " + 4`, `true < 2`, `1 == "1"`). The span is the
    /// two values with their operator.
    BinaryOperands {
        /// The operator.
        operator: BinaryOperator,
        /// The type of the value on its left.
        left: ValueType,
        /// The type of the value on its right.
        right: ValueType,
    },
    /// A side of an `and` or of an `or` that isn't a boolean (`1 and done`). The span is that
    /// side: the other one may not have been evaluated.
    LogicOperand {
        /// `and` or `or`.
        operator: BinaryOperator,
        /// The type of the value of that side.
        found: ValueType,
    },
    /// A division by zero, with `/` or with `%`. The span is the two values with their
    /// operator.
    DivisionByZero,
    /// A result beyond what a whole number holds. The span is the operator with its values.
    Overflow,
    /// Two texts joined into one of more than [`MAX_TEXT_LENGTH`](super::MAX_TEXT_LENGTH)
    /// characters. The span is the two texts with their `+`.
    TextTooLong,
    /// The result of a function that gave back no value, used in an expression
    /// (`let line = print("a")`): the function has run. The span is the call.
    NoValue {
        /// The function.
        function: String,
    },
    /// A call when [`MAX_CALL_DEPTH`](super::MAX_CALL_DEPTH) calls are already in progress,
    /// one inside the other: a recursion that never ends. The span is the call, which
    /// isn't entered.
    TooManyCalls {
        /// The function called.
        function: String,
    },
    /// What the checks before running rule out (a name that is no variable, a `break` outside
    /// a loop…), or a mistake of the core: never for a checked program. The span is the
    /// closest place known.
    Internal,
}
