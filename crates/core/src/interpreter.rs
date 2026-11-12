//! The interpreter: runs a checked program one step at a time (see *Running a program* in
//! `specs.md`, and Interpreter in `docs/technical-choices.md`).
//!
//! The program is first translated into a flat list of instructions (`compiler`), which the
//! [`Machine`] owns and runs: its whole state is data, never the stack of Rust, so that it
//! pauses between two steps anywhere, even in the middle of an expression.
//!
//! The parts depend on each other in one direction: the values (`value`), the errors that
//! name their types (`error`), what the operators compute (`operators`), the instructions
//! (`code`) and their translation (`compiler`), and the machine (`machine`) above them all.

mod code;
mod compiler;
mod error;
mod machine;
mod operators;
mod value;

#[cfg(test)]
mod tests;

pub use error::{RuntimeError, RuntimeErrorKind};
pub use machine::{Event, Machine, State};
pub use value::{MAX_TEXT_LENGTH, Value, ValueType};
