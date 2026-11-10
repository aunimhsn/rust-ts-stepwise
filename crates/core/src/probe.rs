//! The probe of step 4c, thrown away by the interpreter of step 9: a fake program that walks
//! the robot around a square forever, one action per [`Probe::step`], so that the interface
//! tries driving the core at each animation frame, at every speed, before the language exists.

/// The width and the height of the small map of the probe, in cells.
pub const MAP_SIZE: u8 = 5;

/// The cell where the robot starts, facing east, at the top left corner of its square.
const START: (u8, u8) = (1, 1);

/// The fake program, run again from its start once it ends: `move(); move(); turn_right()`.
const PROGRAM: [Instruction; 3] = [Instruction::Move, Instruction::Move, Instruction::TurnRight];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Move,
    TurnRight,
}

/// Where the robot looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turned_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

/// What the robot did during a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// It moved one cell forward.
    Moved,
    /// It turned right, a quarter turn on its cell.
    Turned,
    /// It bumped into the edge of the map and stayed on its cell.
    Bumped,
}

/// The robot on the map: its cell, counted from the top left corner, and its direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    pub x: u8,
    pub y: u8,
    pub direction: Direction,
}

impl Robot {
    /// The cell in front of the robot, or `None` past the edge of the map.
    fn ahead(self) -> Option<(u8, u8)> {
        let (x, y) = match self.direction {
            Direction::North => (Some(self.x), self.y.checked_sub(1)),
            Direction::East => (self.x.checked_add(1), Some(self.y)),
            Direction::South => (Some(self.x), self.y.checked_add(1)),
            Direction::West => (self.x.checked_sub(1), Some(self.y)),
        };
        let inside = |value: u8| value < MAP_SIZE;
        Some((x.filter(|&x| inside(x))?, y.filter(|&y| inside(y))?))
    }
}

/// The fake program running: the robot, the next instruction and the steps run.
#[derive(Debug, Clone)]
pub struct Probe {
    robot: Robot,
    next: usize,
    steps: u64,
}

impl Default for Probe {
    fn default() -> Self {
        Self::new()
    }
}

impl Probe {
    /// The program at its start, the robot on its starting cell.
    #[must_use]
    pub fn new() -> Self {
        Self {
            robot: Robot {
                x: START.0,
                y: START.1,
                direction: Direction::East,
            },
            next: 0,
            steps: 0,
        }
    }

    /// The robot as it is now.
    #[must_use]
    pub fn robot(&self) -> Robot {
        self.robot
    }

    /// The number of steps run since the start, which stops growing at `u64::MAX`.
    #[must_use]
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// Runs the next instruction and tells what the robot did.
    pub fn step(&mut self) -> Action {
        let instruction = PROGRAM.get(self.next).copied().unwrap_or(Instruction::Move);
        self.next = self
            .next
            .checked_add(1)
            .filter(|&next| next < PROGRAM.len())
            .unwrap_or(0);
        self.steps = self.steps.saturating_add(1);
        match instruction {
            Instruction::Move => match self.robot.ahead() {
                Some((x, y)) => {
                    self.robot.x = x;
                    self.robot.y = y;
                    Action::Moved
                }
                None => Action::Bumped,
            },
            Instruction::TurnRight => {
                self.robot.direction = self.robot.direction.turned_right();
                Action::Turned
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_robot_walks_around_its_square_and_comes_back() {
        let mut probe = Probe::new();
        let mut actions = Vec::new();
        for _ in 0..12 {
            actions.push(probe.step());
        }
        let side = [Action::Moved, Action::Moved, Action::Turned];
        assert_eq!(actions, side.repeat(4));
        assert_eq!(probe.robot(), Probe::new().robot());
        assert_eq!(probe.steps(), 12);
    }

    #[test]
    fn the_robot_goes_through_each_corner_of_its_square() {
        let mut probe = Probe::new();
        let mut corners = Vec::new();
        for _ in 0..4 {
            for _ in 0..3 {
                probe.step();
            }
            let robot = probe.robot();
            corners.push((robot.x, robot.y, robot.direction));
        }
        assert_eq!(
            corners,
            [
                (3, 1, Direction::South),
                (3, 3, Direction::West),
                (1, 3, Direction::North),
                (1, 1, Direction::East),
            ]
        );
    }

    #[test]
    fn the_robot_bumps_into_the_edge_of_the_map_and_stays() {
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let last = MAP_SIZE - 1;
            let (x, y) = match direction {
                Direction::North => (2, 0),
                Direction::East => (last, 2),
                Direction::South => (2, last),
                Direction::West => (0, 2),
            };
            let robot = Robot { x, y, direction };
            let mut probe = Probe {
                robot,
                next: 0,
                steps: 0,
            };
            assert_eq!(probe.step(), Action::Bumped);
            assert_eq!(probe.robot(), robot);
        }
    }
}
