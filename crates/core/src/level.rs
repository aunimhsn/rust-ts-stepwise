//! The levels and the sandbox, read from their TOML files (see *Levels and progression* and
//! *Sandbox* in `specs.md`, and Levels in `docs/technical-choices.md`): the starting code, and
//! the maps, each a world with its goals, checked when the file is read so that a level never
//! starts wrong.

mod build;
mod error;
mod file;
mod goals;
mod stroke;

#[cfg(test)]
mod tests;

pub use error::{LevelError, LevelErrorKind};
pub use goals::{GoalNotMet, Goals};
pub use stroke::Stroke;

use crate::world::World;

/// The version of the format of the level files this core reads. It stays 1 until the first
/// release of the website, the files being deployed with the code that reads them; any change
/// of the format after it raises the version.
pub const FORMAT_VERSION: u32 = 1;

/// The most maps a level has: the same task on different ground.
pub const MAX_MAPS: usize = 3;

/// A wall, in the terrain of a map.
pub const WALL: char = '#';

/// A floor cell, in the terrain of a map.
pub const FLOOR: char = '.';

/// A level: the code it starts with, and its maps, which the same program must all pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    starting_code: String,
    maps: Vec<LevelMap>,
}

impl Level {
    /// Reads and checks a level file.
    ///
    /// # Errors
    ///
    /// When the file isn't valid TOML, has another version or the wrong fields, or describes a
    /// map or goals that can't be (see [`LevelErrorKind`]).
    pub fn from_toml(source: &str) -> Result<Self, LevelError> {
        build::level(file::parse(source)?)
    }

    /// The code the learner starts with.
    #[must_use]
    pub fn starting_code(&self) -> &str {
        &self.starting_code
    }

    /// The maps of the level, from one to [`MAX_MAPS`], in the order of the file.
    #[must_use]
    pub fn maps(&self) -> &[LevelMap] {
        &self.maps
    }
}

/// A map of a level: the world as it starts, and the goals to meet on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelMap {
    world: World,
    goals: Goals,
}

impl LevelMap {
    /// The world as it starts.
    #[must_use]
    pub const fn world(&self) -> &World {
        &self.world
    }

    /// The goals to meet.
    #[must_use]
    pub const fn goals(&self) -> &Goals {
        &self.goals
    }
}

/// The sandbox: an open map without goal, and the code it starts with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sandbox {
    starting_code: String,
    world: World,
}

impl Sandbox {
    /// Reads and checks the file of the sandbox: the format of a level, with one `[map]` and no
    /// goals.
    ///
    /// # Errors
    ///
    /// As [`Level::from_toml`], and [`LevelErrorKind::GoalsInSandbox`] when its map has goals.
    pub fn from_toml(source: &str) -> Result<Self, LevelError> {
        build::sandbox(file::parse(source)?)
    }

    /// The code the learner starts with.
    #[must_use]
    pub fn starting_code(&self) -> &str {
        &self.starting_code
    }

    /// The world as it starts.
    #[must_use]
    pub const fn world(&self) -> &World {
        &self.world
    }
}
