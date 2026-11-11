//! The language of Stepwise, run one step at a time.
//!
//! The core must never panic: compiled to WebAssembly, a panic aborts the module and the page
//! with it, while every program of a learner, even absurd, must end with an error the
//! interface can show. The lints below, and the methods forbidden by `clippy.toml`, rule out
//! what can panic outside the tests: indexing, `unwrap`, `panic!`, and the arithmetic, since
//! a division by zero panics and an overflow wraps silently in release (use `checked_*`).
#![cfg_attr(
    not(test),
    deny(
        clippy::arithmetic_side_effects,
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented
    )
)]

pub mod ast;
pub mod builtin;
pub mod checker;
pub mod color;
pub mod lexer;
pub mod parser;
pub mod probe;
pub mod span;

mod word_enum;

#[cfg(test)]
mod test_support;

/// The version of the core, from its `Cargo.toml`.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
