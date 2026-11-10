//! The bridge between the core and the interface: each function exported to JavaScript only
//! converts its arguments and its result, the work stays in `stepwise-core`.
//!
//! The bridge runs in the same WebAssembly module as the core: it must never panic either,
//! under the same lints (Cargo refuses lints added per crate to those of the workspace, hence
//! the copy of the block of `crates/core/src/lib.rs`).
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

use wasm_bindgen::prelude::wasm_bindgen;

/// The version of the core, so that the interface can check which module it loaded.
#[wasm_bindgen(js_name = coreVersion)]
#[must_use]
pub fn core_version() -> String {
    stepwise_core::version().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bridge_gives_the_version_of_the_core() {
        assert_eq!(core_version(), stepwise_core::version());
    }
}
