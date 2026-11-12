//! The errors of a level file. Unlike those of the language, only the authors of the levels
//! see them, in the tests: they are codes with their values, and also a sentence in English
//! (`Display`), without a translation until a level editor exists.

use std::fmt;

use crate::world::{BuildError, MAX_COLUMNS, MAX_ROWS, Point};

use super::{FLOOR, FORMAT_VERSION, MAX_MAPS, WALL};

/// Why a level file can't be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelError {
    /// The map concerned, numbered from 1 in the order of the file, `None` for the file as a
    /// whole (and for the one map of the sandbox).
    pub map: Option<usize>,
    /// What is wrong.
    pub kind: LevelErrorKind,
}

impl LevelError {
    pub(super) const fn file(kind: LevelErrorKind) -> Self {
        Self { map: None, kind }
    }

    pub(super) const fn map(number: usize, kind: LevelErrorKind) -> Self {
        Self {
            map: Some(number),
            kind,
        }
    }
}

/// What is wrong in a level file. The cells are written `[x, y]`, as in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelErrorKind {
    /// Not valid TOML, or a field missing, unknown or of the wrong type, as the TOML reader
    /// says it, with its line and column.
    Syntax(String),
    /// Another version than [`FORMAT_VERSION`].
    Version(u32),
    /// No map, or more than [`MAX_MAPS`].
    MapCount(usize),
    /// A row of the terrain that isn't as long as the first one.
    RowLength {
        /// The row, from 0 at the top.
        y: u8,
        /// Its number of cells.
        length: usize,
        /// The number of cells of the first row.
        expected: usize,
    },
    /// A character of the terrain that is neither [`WALL`] nor [`FLOOR`].
    UnknownCharacter {
        /// Its cell.
        at: Point,
        /// The character.
        character: char,
    },
    /// The map can't be built: its size, or an item, a paint or the robot on a wall or
    /// outside it.
    Map(BuildError),
    /// Two items on the same cell.
    DuplicateItem(Point),
    /// A stroke without any cell.
    EmptyPath,
    /// Two cells that follow each other in the path of a stroke, while they aren't next to
    /// each other.
    NotNeighbors(Point, Point),
    /// A goal on a wall or outside the map.
    Goal(BuildError),
    /// A cell that two strokes of the goals want in two different colors.
    ConflictingPaintGoal(Point),
    /// The same cell twice in the cells where an item should lie.
    DuplicateItemGoal(Point),
    /// A map of a level without any goal: an empty program would pass it.
    NoGoal,
    /// A map whose goals are all met before the program runs: an empty program would pass it.
    GoalsMetAtStart,
    /// More items needed by the goals (on their cells, and carried) than the map has (on its
    /// cells, and carried by the robot).
    NotEnoughItems {
        /// The items the goals need.
        needed: usize,
        /// The items of the map.
        available: usize,
    },
    /// Goals in the map of the sandbox, which has none.
    GoalsInSandbox,
}

impl fmt::Display for LevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(number) = self.map {
            write!(f, "map {number}: ")?;
        }
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for LevelErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(message) => write!(f, "{message}"),
            Self::Version(version) => write!(
                f,
                "the file has version {version}, while this core reads version {FORMAT_VERSION}"
            ),
            Self::MapCount(count) => write!(
                f,
                "the level has {count} maps, while it has from 1 to {MAX_MAPS}"
            ),
            Self::RowLength {
                y,
                length,
                expected,
            } => write!(
                f,
                "row {y} of the terrain has {length} cells, while the first one has {expected}"
            ),
            Self::UnknownCharacter { at, character } => write!(
                f,
                "{} in the terrain is {character:?}, neither a wall `{WALL}` nor a floor `{FLOOR}`",
                cell(*at)
            ),
            Self::Map(error) => write_build_error(f, *error),
            Self::DuplicateItem(at) => write!(f, "two items on {}", cell(*at)),
            Self::EmptyPath => write!(f, "a stroke has an empty path"),
            Self::NotNeighbors(from, to) => write!(
                f,
                "{} and {} follow each other in a path, while they aren't next to each other",
                cell(*from),
                cell(*to)
            ),
            Self::Goal(error) => {
                write!(f, "a goal: ")?;
                write_build_error(f, *error)
            }
            Self::ConflictingPaintGoal(at) => {
                write!(f, "the goals paint {} in two different colors", cell(*at))
            }
            Self::DuplicateItemGoal(at) => {
                write!(f, "the goals want an item on {} twice", cell(*at))
            }
            Self::NoGoal => write!(f, "the map has no goal"),
            Self::GoalsMetAtStart => write!(f, "the goals are all met before the program runs"),
            Self::NotEnoughItems { needed, available } => write!(
                f,
                "the goals need {needed} items, while the map has {available}"
            ),
            Self::GoalsInSandbox => write!(f, "the map of the sandbox has goals"),
        }
    }
}

fn write_build_error(f: &mut fmt::Formatter<'_>, error: BuildError) -> fmt::Result {
    match error {
        BuildError::Size { width, height } => write!(
            f,
            "the terrain is {width} × {height}, while a map has from 1 × 1 to \
             {MAX_COLUMNS} × {MAX_ROWS} cells"
        ),
        BuildError::OutsideMap(at) => write!(f, "{} is outside the map", cell(at)),
        BuildError::OnWall(at) => write!(f, "{} is a wall", cell(at)),
        BuildError::NotEmpty(at) => write!(f, "{} already has a paint or an item", cell(at)),
        BuildError::NotNeighbors(from, to) => write!(
            f,
            "{} and {} follow each other in a path, while they aren't next to each other",
            cell(from),
            cell(to)
        ),
        BuildError::Unpainted(at) => write!(f, "{} has no paint", cell(at)),
    }
}

/// A cell as the file writes it.
fn cell(at: Point) -> String {
    format!("[{}, {}]", at.x, at.y)
}
