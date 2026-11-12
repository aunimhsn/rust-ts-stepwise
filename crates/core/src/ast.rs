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
/// indentation nor a comment at its end. For a statement that opens a block, it is the line
/// up to its `:`, without the block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    /// What the statement is.
    pub kind: StatementKind,
    /// Where it is in the source.
    pub span: Span,
}

impl Statement {
    /// The blocks the statement opens, in the order of the source: those of an `if` and of
    /// its `else if` and `else`, the one of a loop or of a function, none for the others.
    pub fn blocks(&self) -> impl Iterator<Item = &[Self]> {
        let (branches, last): (&[Branch], Option<&[Self]>) = match &self.kind {
            StatementKind::If {
                branches,
                otherwise,
            } => (branches, otherwise.as_deref()),
            StatementKind::While { condition: _, body }
            | StatementKind::Repeat { count: _, body }
            | StatementKind::Function {
                name: _,
                parameters: _,
                body,
            } => (&[], Some(body)),
            StatementKind::Let { .. }
            | StatementKind::Assign { .. }
            | StatementKind::Break
            | StatementKind::Return { .. }
            | StatementKind::Expression(_) => (&[], None),
        };
        branches
            .iter()
            .map(|branch| branch.body.as_slice())
            .chain(last)
    }
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
    /// `if`, with its `else if` and its `else`.
    If {
        /// The `if` and each `else if`, in order, never empty: the block of the first true
        /// condition runs. A list rather than an `if` inside each `else`, so that a long chain of
        /// `else if` doesn't deepen the tree.
        branches: Vec<Branch>,
        /// The block of the `else`, if any.
        otherwise: Option<Vec<Statement>>,
    },
    /// `while condition:`, which runs its block as long as the condition is true.
    While {
        /// The condition, evaluated before each turn.
        condition: Expression,
        /// The block.
        body: Vec<Statement>,
    },
    /// `repeat count:`, which runs its block `count` times.
    Repeat {
        /// The number of turns, evaluated once.
        count: Expression,
        /// The block.
        body: Vec<Statement>,
    },
    /// `function name(parameter, …):`, which defines a function. The parser takes it wherever
    /// it is written: the checker refuses the ones that aren't at the top level.
    Function {
        /// The function defined.
        name: Name,
        /// Its parameters, in order. The checker refuses two of the same name.
        parameters: Vec<Name>,
        /// The block.
        body: Vec<Statement>,
    },
    /// An expression alone on its line. Only a call does something: the checker refuses
    /// the others, since it alone knows whether a name is a function or a variable.
    Expression(Expression),
}

/// An `if` or an `else if` with its block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    /// The condition.
    pub condition: Expression,
    /// The block that runs when the condition is true.
    pub body: Vec<Statement>,
    /// Where its line is in the source, from the `if` or the `else` up to the `:`.
    pub span: Span,
}

/// A name where the language expects one (the variable of a `let`, the function of a call or
/// of a definition, a parameter),
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

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    /// The number of statements of each block of the first statement of `source`.
    fn block_lengths(source: &str) -> Vec<usize> {
        let program = parse(source).unwrap();
        program.statements[0].blocks().map(<[_]>::len).collect()
    }

    #[test]
    fn an_if_has_the_blocks_of_its_branches_in_order() {
        let source = "\
if true:
    forward()
else if false:
    forward()
    forward()
else:
    forward()
    forward()
    forward()
";
        assert_eq!(block_lengths(source), [1, 2, 3]);
        assert_eq!(block_lengths("if true:\n    forward()\n"), [1]);
    }

    #[test]
    fn a_loop_or_a_function_has_one_block() {
        assert_eq!(block_lengths("while true:\n    forward()\n"), [1]);
        assert_eq!(
            block_lengths("repeat 3:\n    forward()\n    forward()\n"),
            [2]
        );
        assert_eq!(block_lengths("function walk():\n    forward()\n"), [1]);
    }

    #[test]
    fn the_blocks_inside_a_block_are_not_among_them() {
        let source = "while true:\n    if true:\n        forward()\n        forward()\n";
        assert_eq!(block_lengths(source), [1]);
    }

    #[test]
    fn the_other_statements_have_none() {
        for source in [
            "let sides = 4",
            "sides = 4",
            "break",
            "return 3",
            "forward()",
        ] {
            assert_eq!(block_lengths(source), [], "{source}");
        }
    }
}
