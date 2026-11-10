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

use stepwise_core::probe;
use wasm_bindgen::prelude::wasm_bindgen;

/// The version of the core, so that the interface can check which module it loaded.
#[wasm_bindgen(js_name = coreVersion)]
#[must_use]
pub fn core_version() -> String {
    stepwise_core::version().to_owned()
}

/// The probe of step 4c, thrown away by the bridge of step 10: the fake program of
/// `stepwise_core::probe`, advanced by the interface at each animation frame.
#[wasm_bindgen]
pub struct Probe(probe::Probe);

/// What the robot did during a step, as `probe::Action`: a string in JavaScript, its type a
/// union of the strings in TypeScript.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Moved = "moved",
    Turned = "turned",
    Bumped = "bumped",
}

/// Where the robot looks, as `probe::Direction`.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = "north",
    East = "east",
    South = "south",
    West = "west",
}

#[wasm_bindgen]
impl Probe {
    /// The program at its start.
    #[wasm_bindgen(constructor)]
    #[must_use]
    #[expect(
        clippy::new_without_default,
        reason = "JavaScript only calls the constructor"
    )]
    pub fn new() -> Self {
        Self(probe::Probe::new())
    }

    /// The width and the height of the map, in cells.
    #[wasm_bindgen(js_name = mapSize)]
    #[must_use]
    pub fn map_size() -> u8 {
        probe::MAP_SIZE
    }

    /// Runs the next instruction.
    pub fn step(&mut self) -> Action {
        match self.0.step() {
            probe::Action::Moved => Action::Moved,
            probe::Action::Turned => Action::Turned,
            probe::Action::Bumped => Action::Bumped,
        }
    }

    /// The column of the robot, from the left.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn x(&self) -> u8 {
        self.0.robot().x
    }

    /// The row of the robot, from the top.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn y(&self) -> u8 {
        self.0.robot().y
    }

    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn direction(&self) -> Direction {
        match self.0.robot().direction {
            probe::Direction::North => Direction::North,
            probe::Direction::East => Direction::East,
            probe::Direction::South => Direction::South,
            probe::Direction::West => Direction::West,
        }
    }

    /// The steps run since the start: a `bigint` in JavaScript.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn steps(&self) -> u64 {
        self.0.steps()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bridge_gives_the_version_of_the_core() {
        assert_eq!(core_version(), stepwise_core::version());
    }

    #[test]
    fn the_probe_converts_the_actions_and_the_robot_of_the_core() {
        let mut probe = Probe::new();
        assert_eq!(
            (probe.x(), probe.y(), probe.direction()),
            (1, 1, Direction::East)
        );
        assert_eq!(
            [probe.step(), probe.step(), probe.step()],
            [Action::Moved, Action::Moved, Action::Turned]
        );
        assert_eq!(
            (probe.x(), probe.y(), probe.direction()),
            (3, 1, Direction::South)
        );
        assert_eq!(probe.steps(), 3);
        assert_eq!(Probe::map_size(), probe::MAP_SIZE);
    }
}
