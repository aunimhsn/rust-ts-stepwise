//! The level files as they are written: their schema, read with `serde`, and their version.
//! What they describe is built by `build.rs`.

use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer};

use super::{FORMAT_VERSION, LevelError, LevelErrorKind};
use crate::color::Color;
use crate::world::{Direction, Point};

/// A file that says the version of its format.
pub(super) trait Versioned {
    fn version(&self) -> u32;
}

/// The version alone, read when a file can't be read whole: a file of another version is
/// refused for its version, rather than for a field this core doesn't know.
#[derive(Deserialize)]
struct Header {
    version: u32,
}

/// A level file.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct LevelFile {
    version: u32,
    pub(super) starting_code: String,
    #[serde(default)]
    pub(super) maps: Vec<MapFile>,
}

impl Versioned for LevelFile {
    fn version(&self) -> u32 {
        self.version
    }
}

/// The file of the sandbox.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SandboxFile {
    version: u32,
    pub(super) starting_code: String,
    pub(super) map: MapFile,
}

impl Versioned for SandboxFile {
    fn version(&self) -> u32 {
        self.version
    }
}

/// A map: its terrain in rows of characters, the rest by coordinates `[x, y]`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MapFile {
    pub(super) terrain: String,
    pub(super) robot: RobotFile,
    #[serde(default)]
    pub(super) items: Vec<[u8; 2]>,
    #[serde(default)]
    pub(super) paints: Vec<StrokeFile>,
    pub(super) goals: Option<GoalsFile>,
}

/// The robot as it starts.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RobotFile {
    pub(super) at: [u8; 2],
    pub(super) facing: Facing,
    #[serde(default)]
    pub(super) carrying: bool,
}

/// A direction, as written.
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(super) enum Facing {
    North,
    East,
    South,
    West,
}

impl From<Facing> for Direction {
    fn from(facing: Facing) -> Self {
        match facing {
            Facing::North => Self::North,
            Facing::East => Self::East,
            Facing::South => Self::South,
            Facing::West => Self::West,
        }
    }
}

/// A stroke of paint: a color and a path of cells.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StrokeFile {
    #[serde(deserialize_with = "color")]
    pub(super) color: Color,
    pub(super) path: Vec<[u8; 2]>,
}

/// The goals of a map; each is optional.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GoalsFile {
    pub(super) robot_at: Option<[u8; 2]>,
    #[serde(default)]
    pub(super) paints: Vec<StrokeFile>,
    #[serde(default)]
    pub(super) nothing_else_painted: bool,
    #[serde(default)]
    pub(super) items_at: Vec<[u8; 2]>,
    pub(super) carrying: Option<bool>,
}

/// Reads a file of TOML, refusing any version of the format but this core's.
pub(super) fn parse<T: DeserializeOwned + Versioned>(source: &str) -> Result<T, LevelError> {
    match toml::from_str::<T>(source) {
        Ok(file) => check_version(file.version()).map(|()| file),
        Err(error) => {
            if let Ok(header) = toml::from_str::<Header>(source) {
                check_version(header.version)?;
            }
            Err(LevelError::file(LevelErrorKind::Syntax(error.to_string())))
        }
    }
}

fn check_version(version: u32) -> Result<(), LevelError> {
    if version == FORMAT_VERSION {
        Ok(())
    } else {
        Err(LevelError::file(LevelErrorKind::Version(version)))
    }
}

/// A cell written `[x, y]`.
pub(super) const fn point([x, y]: [u8; 2]) -> Point {
    Point::new(x, y)
}

/// A color written by its name, as in a program.
fn color<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
    let word = String::deserialize(deserializer)?;
    Color::from_word(&word).ok_or_else(|| {
        let colors: Vec<_> = Color::ALL.into_iter().map(Color::as_str).collect();
        D::Error::custom(format!(
            "unknown color `{word}`, expected one of: {}",
            colors.join(", ")
        ))
    })
}
