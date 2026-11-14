//! A source checked before it runs (see *Checks before running* in `specs.md`): its syntax
//! first, then its names and its structure.
//!
//! [`check`] is the one way to a [`Checked`] program, the only one the machine will take
//! (step 10c): the interface shows the errors of a source that is refused, and will have
//! nothing to start.

use crate::ast::Program;
use crate::checker::{self, CheckError};
use crate::parser::{SyntaxError, parse};

/// A program whose syntax, names and structure are right: only [`check`] gives one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked(Program);

impl Checked {
    /// Its syntax tree.
    #[must_use]
    pub const fn program(&self) -> &Program {
        &self.0
    }
}

/// Why the checks before running refuse a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// Its first syntax error: the rest of the source can't be read reliably after it.
    Syntax(SyntaxError),
    /// Every error of its names and of its structure, in the order of the source, once its
    /// syntax is right: at least one.
    Check(Vec<CheckError>),
}

/// Checks `source`: as the learner types, and again when **Run** is pressed.
///
/// # Errors
///
/// [`Refused`], with the first syntax error of the source or, once its syntax is right, every
/// error of its names and of its structure.
pub fn check(source: &str) -> Result<Checked, Refused> {
    let program = parse(source).map_err(Refused::Syntax)?;
    let errors = checker::check(&program);
    if errors.is_empty() {
        Ok(Checked(program))
    } else {
        Err(Refused::Check(errors))
    }
}

#[cfg(test)]
mod tests {
    use super::{Refused, check};
    use crate::checker::{self, CheckErrorKind};
    use crate::parser::{ParseErrorKind, SyntaxError, parse};

    #[test]
    fn a_source_without_error_gives_its_checked_tree() {
        let source = "let sides = 4\nrepeat sides:\n    forward()\n";
        let checked = check(source).unwrap();
        assert_eq!(*checked.program(), parse(source).unwrap());
    }

    #[test]
    fn a_source_that_does_nothing_is_right() {
        for source in ["", "\n\n", "# Nothing yet\n"] {
            let checked = check(source).unwrap();
            assert_eq!(checked.program().statements, [], "in {source:?}");
        }
    }

    #[test]
    fn a_syntax_error_is_given_alone_whatever_the_names_around_it() {
        // `sidse` is unknown on the line 1, and `turn` on the line 3: the line 2 hides both.
        let source = "sidse = 1\nforward(\nturn()\n";
        let Err(Refused::Syntax(error)) = check(source) else {
            panic!("the syntax should be refused");
        };
        assert_eq!(error, parse(source).unwrap_err());
        let SyntaxError::Parse(error) = error else {
            panic!("an error of the parser: {error:?}");
        };
        assert_eq!(error.kind, ParseErrorKind::UnclosedParen);
        assert_eq!(error.span.start.line, 2);
    }

    #[test]
    fn the_first_error_of_the_lexer_stops_the_checks_too() {
        let Err(Refused::Syntax(error)) = check("sidse = 1\n\tforward()\n") else {
            panic!("the syntax should be refused");
        };
        assert!(matches!(error, SyntaxError::Lex(_)), "{error:?}");
    }

    #[test]
    fn every_error_of_names_is_given_in_the_order_of_the_source() {
        let source = "sidse = 1\nforward()\nturn()\nbreak\n";
        let Err(Refused::Check(errors)) = check(source) else {
            panic!("the names should be refused");
        };
        assert_eq!(errors, checker::check(&parse(source).unwrap()));
        let found: Vec<_> = errors
            .iter()
            .map(|error| (error.span.start.line, &error.kind))
            .collect();
        assert_eq!(
            found,
            [
                (
                    1,
                    &CheckErrorKind::UndeclaredVariableChanged {
                        name: "sidse".to_owned()
                    }
                ),
                (
                    3,
                    &CheckErrorKind::UnknownFunction {
                        name: "turn".to_owned()
                    }
                ),
                (4, &CheckErrorKind::BreakOutsideLoop),
            ]
        );
    }
}
