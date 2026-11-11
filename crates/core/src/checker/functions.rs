//! The functions a program may call: the built-in ones, and its own, which a first pass
//! collects before the walk, so that a name is known as a function wherever it is defined.

use std::collections::HashMap;

use super::ReservedWord;
use crate::ast::{Statement, StatementKind};
use crate::builtin::Builtin;
use crate::span::Span;

/// A function of the program, as its definition gives it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Function {
    /// Where its name is in its definition.
    pub(super) defined: Span,
    /// The number of its parameters, as written: one refused for its name counts too.
    pub(super) parameter_count: usize,
}

/// A function that a call may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Callee {
    /// A built-in function.
    Builtin(Builtin),
    /// A function of the program.
    Defined(Function),
}

impl Callee {
    /// The number of values a call gives it, or `None` when it takes any number.
    pub(super) const fn parameter_count(self) -> Option<usize> {
        match self {
            Self::Builtin(builtin) => builtin.parameter_count(),
            Self::Defined(function) => Some(function.parameter_count),
        }
    }
}

/// The functions of a program, each under its name.
#[derive(Debug, Default)]
pub(super) struct Functions<'a>(HashMap<&'a str, Function>);

impl<'a> Functions<'a> {
    /// The functions defined in `statements` and in their blocks: one written in a block
    /// or in another function is refused, but its name is known all the same.
    pub(super) fn of(statements: &'a [Statement]) -> Self {
        let mut functions = Self::default();
        functions.collect(statements);
        functions
    }

    /// Adds the functions of `statements`, in the order of the source: of two functions of
    /// the same name, the first is the function, as the first `let` is the variable. One
    /// that takes the name of a built-in function or of a color is left out, since that
    /// name keeps its meaning.
    fn collect(&mut self, statements: &'a [Statement]) {
        for statement in statements {
            if let StatementKind::Function {
                name,
                parameters,
                body: _,
            } = &statement.kind
                && !ReservedWord::is_reserved(&name.text)
            {
                self.0.entry(&name.text).or_insert(Function {
                    defined: name.span,
                    parameter_count: parameters.len(),
                });
            }
            for block in statement.blocks() {
                self.collect(block);
            }
        }
    }

    /// The function of the program named `name`, if there is one.
    pub(super) fn get(&self, name: &str) -> Option<Function> {
        self.0.get(name).copied()
    }

    /// The function a call of `name` runs, built-in or of the program, if there is one.
    pub(super) fn callee(&self, name: &str) -> Option<Callee> {
        Builtin::from_word(name)
            .map(Callee::Builtin)
            .or_else(|| self.get(name).map(Callee::Defined))
    }
}
