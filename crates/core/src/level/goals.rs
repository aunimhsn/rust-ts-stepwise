//! The goals of a map, all checked when the program ends without error (see *Goals* in
//! *Levels and progression*, `specs.md`).

use super::{LevelErrorKind, Stroke};
use crate::color::Color;
use crate::world::{BuildError, Cell, Map, Point, World};

/// The goals of a map: each is optional, and a map has at least one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goals {
    robot_at: Option<Point>,
    /// The strokes to paint, kept whole for the marks of the map, which draw their passages.
    paints: Vec<Stroke>,
    /// Each cell of the strokes once, with its color: the goals only look at the cells.
    paint_cells: Vec<(Point, Color)>,
    nothing_else_painted: bool,
    items_at: Vec<Point>,
    carrying: Option<bool>,
}

/// A goal not met at the end of a run, with its cell when it has one, where the map plants
/// its flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalNotMet {
    /// The robot doesn't stand on this cell.
    RobotNotAt(Point),
    /// This cell doesn't have this paint: another one, or none.
    PaintMissing {
        /// The cell.
        at: Point,
        /// The paint it should have.
        color: Color,
    },
    /// This cell is painted, while no other cell than those of the goals should be.
    PaintedElsewhere(Point),
    /// No item lies on this cell.
    ItemMissing(Point),
    /// The robot should carry an item, and carries none.
    NotCarrying,
    /// The robot should carry nothing, and carries an item.
    Carrying,
}

impl GoalNotMet {
    /// The cell of the goal, `None` for what the robot carries, which has no place on the map.
    #[must_use]
    pub const fn cell(self) -> Option<Point> {
        match self {
            Self::RobotNotAt(at)
            | Self::PaintMissing { at, .. }
            | Self::PaintedElsewhere(at)
            | Self::ItemMissing(at) => Some(at),
            Self::NotCarrying | Self::Carrying => None,
        }
    }
}

impl Goals {
    /// The cell where the robot should stand.
    #[must_use]
    pub const fn robot_at(&self) -> Option<Point> {
        self.robot_at
    }

    /// The strokes to paint.
    #[must_use]
    pub fn paints(&self) -> &[Stroke] {
        &self.paints
    }

    /// Whether no cell other than those of the strokes should be painted.
    #[must_use]
    pub const fn nothing_else_painted(&self) -> bool {
        self.nothing_else_painted
    }

    /// The cells where an item should lie.
    #[must_use]
    pub fn items_at(&self) -> &[Point] {
        &self.items_at
    }

    /// Whether the robot should carry an item (`Some(true)`), carry none (`Some(false)`), or
    /// either.
    #[must_use]
    pub const fn carrying(&self) -> Option<bool> {
        self.carrying
    }

    /// Every goal not met on `world`, in the order of the goals: where to stand, the paints,
    /// the cells painted in excess (row by row), the items, what the robot carries. Empty when
    /// every goal is met.
    #[must_use]
    pub fn check(&self, world: &World) -> Vec<GoalNotMet> {
        let map = world.map();
        let robot = world.robot();
        let mut not_met = Vec::new();
        if let Some(at) = self.robot_at
            && robot.position() != at
        {
            not_met.push(GoalNotMet::RobotNotAt(at));
        }
        for &(at, color) in &self.paint_cells {
            if map.paint(at) != Some(color) {
                not_met.push(GoalNotMet::PaintMissing { at, color });
            }
        }
        if self.nothing_else_painted {
            not_met.extend(
                map.points()
                    .filter(|&at| map.paint(at).is_some())
                    .filter(|&at| !self.paint_cells.iter().any(|&(cell, _)| cell == at))
                    .map(GoalNotMet::PaintedElsewhere),
            );
        }
        for &at in &self.items_at {
            if !map.has_item(at) {
                not_met.push(GoalNotMet::ItemMissing(at));
            }
        }
        match self.carrying {
            Some(true) if !robot.is_carrying() => not_met.push(GoalNotMet::NotCarrying),
            Some(false) if robot.is_carrying() => not_met.push(GoalNotMet::Carrying),
            _ => {}
        }
        not_met
    }

    /// The goals of a map of `world` as it starts. Each is optional; a stroke may come back
    /// on a cell of another one in the same color.
    ///
    /// # Errors
    ///
    /// When a goal is on a wall or outside the map, two strokes want a cell in two colors, a
    /// cell is given twice for an item, or the goals are none, need more items than the world
    /// has, or are all met already: no program could pass the map, or an empty one would.
    pub fn new(
        robot_at: Option<Point>,
        paints: Vec<Stroke>,
        nothing_else_painted: bool,
        items_at: Vec<Point>,
        carrying: Option<bool>,
        world: &World,
    ) -> Result<Self, LevelErrorKind> {
        let map = world.map();
        if let Some(at) = robot_at {
            floor(map, at)?;
        }
        let mut paint_cells: Vec<(Point, Color)> = Vec::new();
        for stroke in &paints {
            for &at in stroke.path() {
                floor(map, at)?;
                match paint_cells.iter().find(|&&(cell, _)| cell == at) {
                    Some(&(_, color)) if color != stroke.color() => {
                        return Err(LevelErrorKind::ConflictingPaintGoal(at));
                    }
                    Some(_) => {}
                    None => paint_cells.push((at, stroke.color())),
                }
            }
        }
        for (index, &at) in items_at.iter().enumerate() {
            floor(map, at)?;
            if items_at.iter().take(index).any(|&before| before == at) {
                return Err(LevelErrorKind::DuplicateItemGoal(at));
            }
        }
        let goals = Self {
            robot_at,
            paints,
            paint_cells,
            nothing_else_painted,
            items_at,
            carrying,
        };
        goals.check_possible(world)?;
        Ok(goals)
    }

    /// Refuses goals that are none, that no program can meet, or that an empty program already
    /// meets.
    fn check_possible(&self, world: &World) -> Result<(), LevelErrorKind> {
        if self.robot_at.is_none()
            && self.paints.is_empty()
            && !self.nothing_else_painted
            && self.items_at.is_empty()
            && self.carrying.is_none()
        {
            return Err(LevelErrorKind::NoGoal);
        }
        let map = world.map();
        let needed = self
            .items_at
            .len()
            .saturating_add(usize::from(self.carrying == Some(true)));
        let available = map
            .points()
            .filter(|&at| map.has_item(at))
            .count()
            .saturating_add(usize::from(world.robot().is_carrying()));
        if needed > available {
            return Err(LevelErrorKind::NotEnoughItems { needed, available });
        }
        if self.check(world).is_empty() {
            return Err(LevelErrorKind::GoalsMetAtStart);
        }
        Ok(())
    }
}

/// Refuses a goal on a wall or outside the map.
fn floor(map: &Map, at: Point) -> Result<(), LevelErrorKind> {
    match map.cell(at) {
        Some(Cell::Floor(_)) => Ok(()),
        Some(Cell::Wall) => Err(LevelErrorKind::Goal(BuildError::OnWall(at))),
        None => Err(LevelErrorKind::Goal(BuildError::OutsideMap(at))),
    }
}
