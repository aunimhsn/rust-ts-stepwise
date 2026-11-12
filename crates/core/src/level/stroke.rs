//! A stroke of paint, as a level places it: at the start of a map, or in its goals.

use super::LevelErrorKind;
use crate::color::Color;
use crate::world::Point;

/// A color along a path of cells next to each other: its cells, and the passages between two
/// cells that follow each other. A path may come back on a cell, as a square that closes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stroke {
    color: Color,
    path: Vec<Point>,
}

impl Stroke {
    /// A stroke of `color` along `path`.
    ///
    /// # Errors
    ///
    /// [`LevelErrorKind::EmptyPath`] when the path has no cell,
    /// [`LevelErrorKind::NotNeighbors`] when two cells that follow each other aren't next to
    /// each other.
    pub fn new(color: Color, path: Vec<Point>) -> Result<Self, LevelErrorKind> {
        if path.is_empty() {
            return Err(LevelErrorKind::EmptyPath);
        }
        for (&from, &to) in path.iter().zip(path.iter().skip(1)) {
            if from.direction_to(to).is_none() {
                return Err(LevelErrorKind::NotNeighbors(from, to));
            }
        }
        Ok(Self { color, path })
    }

    /// The color.
    #[must_use]
    pub const fn color(&self) -> Color {
        self.color
    }

    /// The cells, in the order of the path; at least one.
    #[must_use]
    pub fn path(&self) -> &[Point] {
        &self.path
    }

    /// The pairs of cells that follow each other, whose passage the stroke paints.
    pub fn steps(&self) -> impl Iterator<Item = (Point, Point)> {
        self.path
            .iter()
            .copied()
            .zip(self.path.iter().copied().skip(1))
    }
}
