//! The checker: finds the errors of names and of structure of a program whose syntax is
//! right, before it runs (see *Checks before running* in `specs.md`).
//!
//! It walks the syntax tree in the order of the source and keeps every error rather than the
//! first: unlike a syntax error, none of them prevents reading what follows.

mod error;

#[cfg(test)]
mod tests;

pub use error::{CheckError, CheckErrorKind, ReservedWord};

use std::collections::HashSet;

use crate::ast::{Branch, Name, Program, Statement, StatementKind};
use crate::span::Span;

/// Every error of names and of structure of `program`, in the order of the source.
#[must_use]
pub fn check(program: &Program) -> Vec<CheckError> {
    let mut checker = Checker::default();
    checker.block(&program.statements, Place::TOP_LEVEL);
    checker.errors
}

/// Where a statement is in the program: what decides whether a `break`, a `return` or a
/// function may be written there.
#[derive(Debug, Clone, Copy)]
struct Place {
    /// Outside every block and every function.
    top_level: bool,
    /// In the body of a function, however deep in its blocks.
    in_function: bool,
    /// In a `while` or a `repeat` of the same function, or of the main program.
    in_loop: bool,
}

impl Place {
    /// The top level of the program.
    const TOP_LEVEL: Self = Self {
        top_level: true,
        in_function: false,
        in_loop: false,
    };

    /// The body of a function. The loops around the function, if any, are not its own: a
    /// `break` in the function leaves none of them.
    const FUNCTION_BODY: Self = Self {
        top_level: false,
        in_function: true,
        in_loop: false,
    };

    /// A block of an `if` written at this place.
    const fn enter_branch(self) -> Self {
        Self {
            top_level: false,
            ..self
        }
    }

    /// The block of a `while` or of a `repeat` written at this place.
    const fn enter_loop(self) -> Self {
        Self {
            top_level: false,
            in_loop: true,
            ..self
        }
    }
}

/// The errors found so far.
#[derive(Debug, Default)]
struct Checker {
    errors: Vec<CheckError>,
}

impl Checker {
    fn report(&mut self, kind: CheckErrorKind, span: Span) {
        self.errors.push(CheckError { kind, span });
    }

    /// The statements of a block, or of the top level, written at `place`.
    fn block(&mut self, statements: &[Statement], place: Place) {
        for statement in statements {
            self.statement(statement, place);
        }
    }

    fn statement(&mut self, statement: &Statement, place: Place) {
        match &statement.kind {
            StatementKind::Let { name, value: _ } => self.check_new_name(name),
            StatementKind::Assign { name, value: _ } => self.check_not_reserved(name),
            StatementKind::Break => {
                if !place.in_loop {
                    self.report(CheckErrorKind::BreakOutsideLoop, statement.span);
                }
            }
            StatementKind::Return { value: _ } => {
                if !place.in_function {
                    self.report(CheckErrorKind::ReturnOutsideFunction, statement.span);
                }
            }
            StatementKind::If {
                branches,
                otherwise,
            } => {
                for Branch {
                    condition: _,
                    body,
                    span: _,
                } in branches
                {
                    self.block(body, place.enter_branch());
                }
                if let Some(body) = otherwise {
                    self.block(body, place.enter_branch());
                }
            }
            StatementKind::While { condition: _, body }
            | StatementKind::Repeat { count: _, body } => {
                self.block(body, place.enter_loop());
            }
            StatementKind::Function {
                name,
                parameters,
                body,
            } => {
                if !place.top_level {
                    self.report(CheckErrorKind::NestedFunction, statement.span);
                }
                self.check_new_name(name);
                self.parameters(parameters);
                self.block(body, Place::FUNCTION_BODY);
            }
            StatementKind::Expression(_) => {}
        }
    }

    /// The parameters of a function: each a new name, none twice.
    fn parameters(&mut self, parameters: &[Name]) {
        let mut seen = HashSet::new();
        for parameter in parameters {
            self.check_new_name(parameter);
            if !seen.insert(parameter.text.as_str()) {
                let name = parameter.text.clone();
                self.report(CheckErrorKind::DuplicateParameter { name }, parameter.span);
            }
        }
    }

    /// A name that a `let`, a function or a parameter brings into the program: in
    /// `snake_case`, and not reserved.
    fn check_new_name(&mut self, name: &Name) {
        if !is_snake_case(&name.text) {
            self.report(CheckErrorKind::NotSnakeCase, name.span);
        }
        self.check_not_reserved(name);
    }

    /// A name that is given a value: not the one of a built-in function nor of a color.
    fn check_not_reserved(&mut self, name: &Name) {
        if let Some(word) = ReservedWord::from_word(&name.text) {
            self.report(CheckErrorKind::ReservedName { word }, name.span);
        }
    }
}

/// Whether `name` is written as the names of the language: lowercase letters from `a` to
/// `z`, digits and underscores, starting with a letter.
fn is_snake_case(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}
