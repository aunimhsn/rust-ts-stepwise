//! The interpreter: runs a checked program one step at a time (see *Running a program* in
//! `specs.md`, and Interpreter in `docs/technical-choices.md`).
//!
//! The program is first translated into a flat list of instructions (`compiler`), which the
//! [`Machine`] owns and runs: its whole state is data, never the stack of Rust, so that it
//! pauses between two steps anywhere, even in the middle of an expression, and so that the
//! calls in progress and their variables are read as they are ([`Machine::calls`]).
//!
//! The parts depend on each other in one direction: the values (`value`), the errors that
//! name their types (`error`), what the operators compute (`operators`), the instructions
//! (`code`) and their translation (`compiler`), what a step reports (`event`), what the
//! built-in functions do to the world of the robot (`builtins`), what each instruction
//! changes (`memory`), and the machine (`machine`) above them all, which runs the
//! instructions up to the end of a step. The program acts on the world through `builtins`
//! alone; the machine knows nothing of the goals of a level, which are checked on its world
//! once the program has ended.

mod builtins;
mod code;
mod compiler;
mod error;
mod event;
mod machine;
mod memory;
mod operators;
mod value;

#[cfg(test)]
mod tests;

pub use error::{RuntimeError, RuntimeErrorKind};
pub use event::{Action, Event};
pub use machine::{Machine, State};
pub use memory::{CallInProgress, MAX_CALL_DEPTH, Variable};
pub use value::{MAX_TEXT_LENGTH, Value, ValueType};
