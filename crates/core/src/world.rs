//! The world of the robot: the map, and the robot with its brush and its eraser. Every action
//! of a built-in function and every sensor is a method of [`World`] (see *Built-in functions*
//! and *Map* in `specs.md`), so that the interpreter only calls them.

mod map;

#[cfg(test)]
mod tests;

pub use map::{BuildError, Cell, Direction, Floor, MAX_COLUMNS, MAX_ROWS, Map, Point};

use crate::color::Color;

/// The tool the robot has down: at most one of them, as putting one down lifts the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// The brush, with its color: the robot paints every cell it moves into.
    Brush(Color),
    /// The eraser: the robot removes the paint of every cell it moves into.
    Eraser,
}

/// The robot: where it stands, where it faces, what it carries and which tool is down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    position: Point,
    direction: Direction,
    carrying: bool,
    tool: Option<Tool>,
}

impl Robot {
    /// A robot at `position`, facing `direction`, carrying nothing, with its brush and its
    /// eraser up.
    #[must_use]
    pub const fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
            carrying: false,
            tool: None,
        }
    }

    /// The same robot, carrying an item.
    #[must_use]
    pub const fn carrying(self) -> Self {
        Self {
            carrying: true,
            ..self
        }
    }

    /// The cell where the robot stands.
    #[must_use]
    pub const fn position(self) -> Point {
        self.position
    }

    /// The direction the robot faces.
    #[must_use]
    pub const fn direction(self) -> Direction {
        self.direction
    }

    /// Whether the robot carries an item.
    #[must_use]
    pub const fn is_carrying(self) -> bool {
        self.carrying
    }

    /// The tool that is down, if any.
    #[must_use]
    pub const fn tool(self) -> Option<Tool> {
        self.tool
    }
}

/// A side of the robot, for the sensors: in front, on its left or on its right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// In front.
    Front,
    /// On its left.
    Left,
    /// On its right.
    Right,
}

/// Why an action of the robot fails: the program stops with this error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotError {
    /// `forward()` with a wall or the edge of the map in front: the robot bumps into it.
    Bump,
    /// `pick_up()` while the robot already carries an item.
    AlreadyCarrying,
    /// `pick_up()` on a cell without item.
    NoItemHere,
    /// `drop()` while the robot carries nothing.
    NotCarrying,
    /// `drop()` on a cell that already has an item.
    ItemAlreadyHere,
}

/// The map and the robot on it, changed only by the actions of the robot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    map: Map,
    robot: Robot,
}

impl World {
    /// The world of `map` with `robot` on it.
    ///
    /// # Errors
    ///
    /// When the robot is outside the map or on a wall.
    pub fn new(map: Map, robot: Robot) -> Result<Self, BuildError> {
        let position = robot.position;
        match map.cell(position) {
            Some(Cell::Floor(_)) => Ok(Self { map, robot }),
            Some(Cell::Wall) => Err(BuildError::OnWall(position)),
            None => Err(BuildError::OutsideMap(position)),
        }
    }

    /// The map.
    #[must_use]
    pub const fn map(&self) -> &Map {
        &self.map
    }

    /// The robot.
    #[must_use]
    pub const fn robot(&self) -> Robot {
        self.robot
    }

    /// `forward()`: moves the robot one cell in front. With the brush down, it paints that
    /// cell and the passage it crosses; with the eraser down, it removes the paint of that
    /// cell and of its passages.
    ///
    /// # Errors
    ///
    /// [`RobotError::Bump`] when a wall or the edge of the map is in front: the robot doesn't
    /// move.
    pub fn forward(&mut self) -> Result<(), RobotError> {
        let from = self.robot.position;
        let to = self
            .toward(Side::Front)
            .filter(|&to| self.map.is_floor(to))
            .ok_or(RobotError::Bump)?;
        self.robot.position = to;
        match self.robot.tool {
            Some(Tool::Brush(color)) => self.map.paint_step(from, to, color),
            Some(Tool::Eraser) => self.map.erase(to),
            None => {}
        }
        Ok(())
    }

    /// `turn_left()`: turns the robot a quarter turn to its left, on its cell.
    pub const fn turn_left(&mut self) {
        self.robot.direction = self.robot.direction.turned_left();
    }

    /// `turn_right()`: turns the robot a quarter turn to its right, on its cell.
    pub const fn turn_right(&mut self) {
        self.robot.direction = self.robot.direction.turned_right();
    }

    /// `start_painting(color)`: puts the brush down with `color`, lifting the eraser, and
    /// paints the cell of the robot at once, over any paint already there. Called while
    /// painting, it changes the color.
    pub fn start_painting(&mut self, color: Color) {
        self.robot.tool = Some(Tool::Brush(color));
        self.map.paint_cell(self.robot.position, color);
    }

    /// `stop_painting()`: lifts the brush; nothing happens when it is already up.
    pub const fn stop_painting(&mut self) {
        if let Some(Tool::Brush(_)) = self.robot.tool {
            self.robot.tool = None;
        }
    }

    /// `start_erasing()`: puts the eraser down, lifting the brush, and removes at once the
    /// paint of the cell of the robot and of its passages.
    pub fn start_erasing(&mut self) {
        self.robot.tool = Some(Tool::Eraser);
        self.map.erase(self.robot.position);
    }

    /// `stop_erasing()`: lifts the eraser; nothing happens when it is already up.
    pub const fn stop_erasing(&mut self) {
        if let Some(Tool::Eraser) = self.robot.tool {
            self.robot.tool = None;
        }
    }

    /// `pick_up()`: picks up the item of the cell of the robot.
    ///
    /// # Errors
    ///
    /// [`RobotError::AlreadyCarrying`] when the robot carries an item, else
    /// [`RobotError::NoItemHere`] when the cell has none. The state of the robot is checked
    /// first: it explains a `pick_up()` called twice.
    pub fn pick_up(&mut self) -> Result<(), RobotError> {
        if self.robot.carrying {
            return Err(RobotError::AlreadyCarrying);
        }
        if !self.item_here() {
            return Err(RobotError::NoItemHere);
        }
        self.map.put_item(self.robot.position, false);
        self.robot.carrying = true;
        Ok(())
    }

    /// `drop()`: drops the item the robot carries on its cell.
    ///
    /// # Errors
    ///
    /// [`RobotError::NotCarrying`] when the robot carries nothing, else
    /// [`RobotError::ItemAlreadyHere`] when the cell has an item. The state of the robot is
    /// checked first: it explains a `drop()` called twice.
    pub fn drop_item(&mut self) -> Result<(), RobotError> {
        if !self.robot.carrying {
            return Err(RobotError::NotCarrying);
        }
        if self.item_here() {
            return Err(RobotError::ItemAlreadyHere);
        }
        self.map.put_item(self.robot.position, true);
        self.robot.carrying = false;
        Ok(())
    }

    /// `front_is_clear()`, `left_is_clear()`, `right_is_clear()`: whether the cell on that
    /// side is a floor cell of the map.
    #[must_use]
    pub fn is_clear(&self, side: Side) -> bool {
        self.toward(side)
            .is_some_and(|point| self.map.is_floor(point))
    }

    /// The paint of the cell of the robot, for `here_is(color)`.
    #[must_use]
    pub fn paint_here(&self) -> Option<Color> {
        self.map.paint(self.robot.position)
    }

    /// The paint of the cell on that side, `None` for a wall or the edge of the map, for
    /// `front_is(color)`, `left_is(color)` and `right_is(color)`.
    #[must_use]
    pub fn paint_toward(&self, side: Side) -> Option<Color> {
        self.map.paint(self.toward(side)?)
    }

    /// `item_here()`: whether the cell of the robot has an item.
    #[must_use]
    pub fn item_here(&self) -> bool {
        self.map.has_item(self.robot.position)
    }

    /// `item_in_front()`: whether the cell in front has an item.
    #[must_use]
    pub fn item_in_front(&self) -> bool {
        self.toward(Side::Front)
            .is_some_and(|point| self.map.has_item(point))
    }

    /// `is_carrying()`: whether the robot carries an item.
    #[must_use]
    pub const fn is_carrying(&self) -> bool {
        self.robot.carrying
    }

    /// `is_painting()`: whether the brush is down.
    #[must_use]
    pub const fn is_painting(&self) -> bool {
        matches!(self.robot.tool, Some(Tool::Brush(_)))
    }

    /// `is_erasing()`: whether the eraser is down.
    #[must_use]
    pub const fn is_erasing(&self) -> bool {
        matches!(self.robot.tool, Some(Tool::Eraser))
    }

    /// The cell on `side` of the robot, `None` past the top or the left of the map.
    fn toward(&self, side: Side) -> Option<Point> {
        let direction = self.robot.direction;
        let direction = match side {
            Side::Front => direction,
            Side::Left => direction.turned_left(),
            Side::Right => direction.turned_right(),
        };
        self.robot.position.toward(direction)
    }
}
