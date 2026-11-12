//! The checker: finds the errors of names and of structure of a program whose syntax is
//! right, before it runs (see *Checks before running* in `specs.md`).
//!
//! It walks the syntax tree in the order of the source and keeps every error rather than the
//! first: unlike a syntax error, none of them prevents reading what follows.

mod error;
mod functions;
mod scope;

#[cfg(test)]
mod tests;

pub use error::{CheckError, CheckErrorKind, ReservedWord};

use std::collections::HashSet;

use crate::ast::{Branch, Expression, ExpressionKind, Name, Program, Statement, StatementKind};
use crate::color::Color;
use crate::span::Span;
use functions::{Callee, Function, Functions};
use scope::{Missing, Variables, main_program_variables};

/// Every error of names and of structure of `program`, in the order of the source.
#[must_use]
pub fn check(program: &Program) -> Vec<CheckError> {
    let mut checker = Checker {
        main_variables: main_program_variables(&program.statements),
        functions: Functions::of(&program.statements),
        ..Checker::default()
    };
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

/// Whether a variable is read or given a new value: what is said of a name that is nowhere.
#[derive(Debug, Clone, Copy)]
enum Access {
    Read,
    Change,
}

/// The errors found so far, and the variables known at the place the walk has reached.
#[derive(Debug, Default)]
struct Checker<'a> {
    errors: Vec<CheckError>,
    /// The variables of the main program, or of the function being checked.
    variables: Variables<'a>,
    /// Every variable of the main program, wherever its `let` is: what a function doesn't see.
    main_variables: HashSet<&'a str>,
    /// Every function of the program, wherever it is defined.
    functions: Functions<'a>,
}

impl<'a> Checker<'a> {
    fn report(&mut self, kind: CheckErrorKind, span: Span) {
        self.errors.push(CheckError { kind, span });
    }

    /// The statements of a block, or of the top level, written at `place`.
    fn block(&mut self, statements: &'a [Statement], place: Place) {
        self.variables.open(statements);
        for statement in statements {
            self.statement(statement, place);
        }
        self.variables.close();
    }

    fn statement(&mut self, statement: &'a Statement, place: Place) {
        match &statement.kind {
            StatementKind::Let { name, value } => self.declaration(name, value, place),
            StatementKind::Assign { name, value } => {
                if self.check_variable_name(name) {
                    self.variable(&name.text, name.span, Access::Change, place);
                }
                self.expression(value, place);
            }
            StatementKind::Break => {
                if !place.in_loop {
                    self.report(CheckErrorKind::BreakOutsideLoop, statement.span);
                }
            }
            StatementKind::Return { value } => {
                if !place.in_function {
                    self.report(CheckErrorKind::ReturnOutsideFunction, statement.span);
                }
                if let Some(value) = value {
                    self.expression(value, place);
                }
            }
            StatementKind::If {
                branches,
                otherwise,
            } => {
                for Branch {
                    condition,
                    body,
                    span: _,
                } in branches
                {
                    self.expression(condition, place);
                    self.block(body, place.enter_branch());
                }
                if let Some(body) = otherwise {
                    self.block(body, place.enter_branch());
                }
            }
            StatementKind::While {
                condition: header,
                body,
            }
            | StatementKind::Repeat {
                count: header,
                body,
            } => {
                self.expression(header, place);
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
                self.check_function_name(name);
                self.function(parameters, body);
            }
            StatementKind::Expression(expression) => {
                if !self.is_call_or_function(expression) {
                    self.report(CheckErrorKind::ExpressionAlone, statement.span);
                }
                self.expression(expression, place);
            }
        }
    }

    /// A `let`. Its value is read before the variable exists (`let x = x + 1`), so the name
    /// is checked first, in the order of the source, and declared last.
    fn declaration(&mut self, name: &'a Name, value: &'a Expression, place: Place) {
        let free = self.check_new_variable(name);
        let first = self.variables.declared(&name.text);
        if let (true, Some(first)) = (free, first) {
            let name_text = name.text.clone();
            self.report(
                CheckErrorKind::AlreadyDeclared {
                    name: name_text,
                    first,
                },
                name.span,
            );
        }
        self.expression(value, place);
        if free && first.is_none() {
            self.variables.declare(&name.text, name.span);
        }
    }

    /// The parameters and the body of a function, which sees only its own variables: those
    /// of the place where it is written are put aside while it is checked.
    fn function(&mut self, parameters: &'a [Name], body: &'a [Statement]) {
        let around = std::mem::take(&mut self.variables);
        // The parameters are variables of a block around the body: a `let` of the body
        // can't take their names.
        self.variables.open(&[]);
        self.parameters(parameters);
        self.block(body, Place::FUNCTION_BODY);
        let function = std::mem::replace(&mut self.variables, around);
        self.variables.absorb(function);
    }

    /// The parameters of a function: each a new name, none twice. A name that is reserved or
    /// that a function has keeps its one error, even written twice: it is no parameter.
    fn parameters(&mut self, parameters: &'a [Name]) {
        let mut seen = HashSet::new();
        for parameter in parameters {
            if !self.check_new_variable(parameter) {
                continue;
            }
            if seen.insert(parameter.text.as_str()) {
                self.variables.declare(&parameter.text, parameter.span);
            } else {
                let name = parameter.text.clone();
                self.report(CheckErrorKind::DuplicateParameter { name }, parameter.span);
            }
        }
    }

    /// Whether an expression alone on its line is a call, the one expression that does
    /// something, or a function named without its parentheses, whose own error says enough:
    /// the line does something once they are written.
    fn is_call_or_function(&self, expression: &Expression) -> bool {
        match &expression.kind {
            ExpressionKind::Call { .. } => true,
            ExpressionKind::Name(name) => self.functions.callee(name).is_some(),
            ExpressionKind::Number(_)
            | ExpressionKind::Boolean(_)
            | ExpressionKind::Text(_)
            | ExpressionKind::Unary { .. }
            | ExpressionKind::Binary { .. } => false,
        }
    }

    /// The names an expression reads and the functions it calls.
    fn expression(&mut self, expression: &'a Expression, place: Place) {
        match &expression.kind {
            ExpressionKind::Number(_) | ExpressionKind::Boolean(_) | ExpressionKind::Text(_) => {}
            ExpressionKind::Name(name) => {
                if Color::from_word(name).is_some() {
                    // A color is a value.
                } else if self.functions.callee(name).is_some() {
                    let name = name.clone();
                    self.report(
                        CheckErrorKind::FunctionWithoutParentheses { name },
                        expression.span,
                    );
                } else {
                    self.variable(name, expression.span, Access::Read, place);
                }
            }
            ExpressionKind::Call {
                function,
                arguments,
            } => {
                self.call(function, arguments.len());
                for argument in arguments {
                    self.expression(argument, place);
                }
            }
            ExpressionKind::Unary {
                operator: _,
                operand,
            } => self.expression(operand, place),
            ExpressionKind::Binary {
                operator: _,
                left,
                right,
            } => {
                self.expression(left, place);
                self.expression(right, place);
            }
        }
    }

    /// A variable read or changed at `span`: it must exist there.
    fn variable(&mut self, name: &str, span: Span, access: Access, place: Place) {
        let Some(missing) = self.variables.missing(name) else {
            return;
        };
        let of_main_program = place.in_function && self.main_variables.contains(name);
        // The name is copied only now that there is an error to hold it.
        let name = name.to_owned();
        let kind = match missing {
            Missing::BeforeLet(declared) => CheckErrorKind::VariableBeforeLet { name, declared },
            Missing::AfterBlock(declared) => CheckErrorKind::VariableAfterBlock { name, declared },
            Missing::Unknown if of_main_program => CheckErrorKind::MainVariableInFunction { name },
            Missing::Unknown => match access {
                Access::Read => CheckErrorKind::UnknownName { name },
                Access::Change => CheckErrorKind::UndeclaredVariableChanged { name },
            },
        };
        self.report(kind, span);
    }

    /// A call of `function` with `given` values: the name is a function, defined above the
    /// call when it is one of the program, that has as many parameters. Every error of a
    /// call is on its name.
    fn call(&mut self, function: &Name, given: usize) {
        // The name is copied only when there is an error to hold it.
        let name = || function.text.clone();
        let Some(callee) = self.functions.callee(&function.text) else {
            // Only a variable that exists here is known to be one: any other name is no
            // function, whatever it is elsewhere.
            let kind = if self.variables.declared(&function.text).is_some() {
                CheckErrorKind::VariableCalled { name: name() }
            } else {
                CheckErrorKind::UnknownFunction { name: name() }
            };
            self.report(kind, function.span);
            return;
        };
        // The order of the source is enough: a call in the body of the function itself
        // (a recursion) is under its name.
        if let Callee::Defined(Function { defined, .. }) = callee
            && function.span.start < defined.start
        {
            let name = name();
            self.report(
                CheckErrorKind::CalledBeforeDefinition { name, defined },
                function.span,
            );
        }
        if let Some(expected) = callee.parameter_count()
            && expected != given
        {
            let name = name();
            self.report(
                CheckErrorKind::WrongArgumentCount {
                    name,
                    expected,
                    given,
                },
                function.span,
            );
        }
    }

    /// The name of a function in its definition: in `snake_case`, not reserved, and not the
    /// one of a function defined above.
    fn check_function_name(&mut self, name: &Name) {
        self.check_snake_case(name);
        let kind = match ReservedWord::from_word(&name.text) {
            Some(word) => CheckErrorKind::ReservedName { word },
            None => match self.functions.get(&name.text) {
                Some(Function { defined: first, .. }) if first != name.span => {
                    let name = name.text.clone();
                    CheckErrorKind::DuplicateFunction { name, first }
                }
                _ => return,
            },
        };
        self.report(kind, name.span);
    }

    /// A name that a `let` or a parameter brings into the program: in `snake_case`, and
    /// free for a variable. Whether the name is free.
    fn check_new_variable(&mut self, name: &Name) -> bool {
        self.check_snake_case(name);
        self.check_variable_name(name)
    }

    /// A name that a `let`, a function or a parameter brings into the program: written as
    /// the names of the language.
    fn check_snake_case(&mut self, name: &Name) {
        if !is_snake_case(&name.text) {
            self.report(CheckErrorKind::NotSnakeCase, name.span);
        }
    }

    /// A name that is given a value: not the one of a built-in function, of a color nor of
    /// a function of the program. Whether the name is free. A name that isn't has this one
    /// error: it is no variable, so nothing more is said of it.
    fn check_variable_name(&mut self, name: &Name) -> bool {
        let kind = if let Some(word) = ReservedWord::from_word(&name.text) {
            CheckErrorKind::ReservedName { word }
        } else if let Some(Function { defined, .. }) = self.functions.get(&name.text) {
            let name = name.text.clone();
            CheckErrorKind::NameOfFunction { name, defined }
        } else {
            return true;
        };
        self.report(kind, name.span);
        false
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
