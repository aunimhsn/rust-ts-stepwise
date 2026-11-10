//! The syntax tree of a program: what the parser builds, the checker reads and the
//! interpreter runs.
//!
//! Every node keeps the stretch of the source it comes from, for the errors and for the
//! current line shown while the program runs. The tree is never deeper than
//! [`MAX_DEPTH`](crate::parser::MAX_DEPTH): whoever walks it may do so with a recursion.

use crate::span::Span;

/// A whole program: its statements, in the order of the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    /// The statements of the top level.
    pub statements: Vec<Statement>,
}

/// A statement, with the stretch of the source it comes from: its line, without the
/// indentation nor a comment at its end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    /// What the statement is.
    pub kind: StatementKind,
    /// Where it is in the source.
    pub span: Span,
}

/// What a statement is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementKind {
    /// `let name = value`, which declares a variable.
    Let {
        /// The variable declared.
        name: Name,
        /// Its first value.
        value: Expression,
    },
    /// `name = value`, which changes a variable.
    Assign {
        /// The variable changed.
        name: Name,
        /// Its new value.
        value: Expression,
    },
    /// `break`
    Break,
    /// `return` or `return value`
    Return {
        /// The value given back, if any.
        value: Option<Expression>,
    },
    /// An expression alone on its line. Only a call does something: the checker refuses
    /// the others, since it alone knows whether a name is a function or a variable.
    Expression(Expression),
}

/// A name where the language expects one (the variable of a `let`, the function of a call),
/// as written: the checker refuses the names that aren't in `snake_case` or are reserved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    /// The name.
    pub text: String,
    /// Where it is in the source.
    pub span: Span,
}

/// An expression, with the stretch of the source it comes from, its parentheses included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    /// What the expression is.
    pub kind: ExpressionKind,
    /// Where it is in the source.
    pub span: Span,
}

/// What an expression is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    /// A whole number, as written: never negative, since `-7` is a [`UnaryOperator::Negate`].
    Number(i64),
    /// `true` or `false`
    Boolean(bool),
    /// A text, its escapes already replaced.
    Text(String),
    /// A name alone: a variable, a color, or a function written without its parentheses.
    Name(String),
    /// A call: `paint_side(3)`.
    Call {
        /// The function called.
        function: Name,
        /// The values given, in order.
        arguments: Vec<Expression>,
    },
    /// An operator in front of a value: `-length`, `not done`.
    Unary {
        /// The operator.
        operator: UnaryOperator,
        /// The value it applies to.
        operand: Box<Expression>,
    },
    /// An operator between two values: `sides + 1`.
    Binary {
        /// The operator.
        operator: BinaryOperator,
        /// The value on its left.
        left: Box<Expression>,
        /// The value on its right.
        right: Box<Expression>,
    },
}

/// The operators written in front of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    /// `-`
    Negate,
    /// `not`
    Not,
}

/// The operators written between two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `%`
    Remainder,
    /// `==`
    Equal,
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
    /// `and`
    And,
    /// `or`
    Or,
}

impl BinaryOperator {
    /// Whether the operator compares two values: comparisons don't chain.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Equal
                | Self::NotEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
        )
    }
}
