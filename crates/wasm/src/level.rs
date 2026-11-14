//! The levels and the sandbox, for the interface (see *Levels and progression* and *Sandbox*
//! in `specs.md`): [`Level`] and [`Sandbox`] read a file and keep what the core checked in it,
//! and give as data the code it starts with, the world of each map and the goals to mark.

use serde::Serialize;
use stepwise_core::level;
use tsify::{Ts, Tsify};
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::world::{Color, Direction, Point, World, passages};

/// A level, read from its file and checked once. It stays in the module, where a program
/// will run on its maps: the interface holds it while its screen is open, then frees it.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Level(level::Level);

#[wasm_bindgen]
impl Level {
    /// Reads and checks a level file.
    ///
    /// # Errors
    ///
    /// It throws when the file isn't valid, with the sentence of the core that says why, in
    /// English, for the author of the level (`map 2: [3, 1] is a wall`).
    #[wasm_bindgen(constructor)]
    pub fn new(toml: &str) -> Result<Self, JsError> {
        level::Level::from_toml(toml)
            .map(Self)
            .map_err(|error| refused(&error))
    }

    /// The code the learner starts with.
    #[wasm_bindgen(js_name = startingCode)]
    #[must_use]
    pub fn starting_code(&self) -> String {
        self.0.starting_code().to_owned()
    }

    /// The maps of the level, from one to three, in the order of the file: each with its
    /// world as it starts and its goals to mark. They are built again at each call.
    ///
    /// # Errors
    ///
    /// It throws when a map can't be made a JavaScript object, which none of them does.
    pub fn maps(&self) -> Result<Vec<Ts<LevelMap>>, JsError> {
        self.0
            .maps()
            .iter()
            .map(|map| Ok(Ts::from_rust(&LevelMap::from(map))?))
            .collect()
    }
}

/// The sandbox, read from its file and checked once: an open map without goal. It stays in
/// the module, as a level.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Sandbox(level::Sandbox);

#[wasm_bindgen]
impl Sandbox {
    /// Reads and checks the file of the sandbox.
    ///
    /// # Errors
    ///
    /// It throws when the file isn't valid, with the sentence of the core, as for a level.
    #[wasm_bindgen(constructor)]
    pub fn new(toml: &str) -> Result<Self, JsError> {
        level::Sandbox::from_toml(toml)
            .map(Self)
            .map_err(|error| refused(&error))
    }

    /// The code the learner starts with.
    #[wasm_bindgen(js_name = startingCode)]
    #[must_use]
    pub fn starting_code(&self) -> String {
        self.0.starting_code().to_owned()
    }

    /// The world of the sandbox as it starts. It is built again at each call.
    ///
    /// # Errors
    ///
    /// It throws when the world can't be made a JavaScript object, which it never does.
    pub fn world(&self) -> Result<Ts<World>, JsError> {
        Ok(Ts::from_rust(&World::from(self.0.world()))?)
    }
}

/// The error thrown for a file the core refuses: its sentence, as the tests of the core
/// print it.
fn refused(error: &level::LevelError) -> JsError {
    JsError::new(&error.to_string())
}

/// A map of a level: the world as it starts, and the goals to mark on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
pub struct LevelMap {
    /// The world as it starts.
    pub world: World,
    /// The goals the map marks.
    pub goals: Goals,
}

/// The goals of a map that have a cell, where the map marks them before the program runs.
/// The two others have none, and only the text of the goal says them: that no other cell is
/// painted, and what the robot carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct Goals {
    /// The cell where the robot should stand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_at: Option<Point>,
    /// The cells to paint, each once, in the order of the strokes of the file.
    pub paints: Vec<PaintGoal>,
    /// The cells where an item should lie.
    pub items_at: Vec<Point>,
}

/// A cell to paint, drawn as the paint of a cell of the world (`Paint`), which it has the
/// shape of: the goals only look at its color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Tsify)]
pub struct PaintGoal {
    /// The cell.
    pub at: Point,
    /// The color it should have.
    pub color: Color,
    /// The passages the strokes of the goals cross from the cell, clockwise from the north.
    pub passages: Vec<Direction>,
}

impl From<&level::LevelMap> for LevelMap {
    fn from(map: &level::LevelMap) -> Self {
        Self {
            world: map.world().into(),
            goals: map.goals().into(),
        }
    }
}

impl From<&level::Goals> for Goals {
    fn from(goals: &level::Goals) -> Self {
        let paints = goals.paint_cells().iter().map(|&(at, color)| PaintGoal {
            at: at.into(),
            color: color.into(),
            passages: passages(|direction| goals.passage(at, direction)),
        });
        Self {
            robot_at: goals.robot_at().map(Point::from),
            paints: paints.collect(),
            items_at: goals.items_at().iter().copied().map(Point::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use stepwise_core::level;

    use super::{Goals, Level, LevelMap, PaintGoal, Sandbox};
    use crate::world::Direction::{self, East, North, South, West};
    use crate::world::{Color, Point, World};

    /// A level of two maps: a square to paint, then a corridor with an item to carry.
    const LEVEL: &str = r#"
version = 1

starting_code = '''
# Paint a square
'''

[[maps]]
terrain = '''
...
...
'''
robot = { at = [0, 0], facing = "east" }

[maps.goals]
robot_at = [0, 0]
paints = [
    { color = "red", path = [[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]] },
    { color = "blue", path = [[2, 1]] },
]
nothing_else_painted = true

[[maps]]
terrain = '''
#....
'''
robot = { at = [1, 0], facing = "east" }
items = [[2, 0], [3, 0]]

[maps.goals]
items_at = [[4, 0], [3, 0]]
carrying = false
"#;

    const SANDBOX: &str = r#"
version = 1

starting_code = '''
# Try anything here.
'''

[map]
terrain = '''
.#
'''
robot = { at = [0, 0], facing = "north" }
"#;

    const fn at(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    #[track_caller]
    fn maps(source: &str) -> Vec<LevelMap> {
        let level = level::Level::from_toml(source).expect("a valid level file");
        level.maps().iter().map(LevelMap::from).collect()
    }

    #[test]
    fn a_level_gives_its_starting_code_as_written() {
        let level = Level::new(LEVEL).expect("a valid level file");
        assert_eq!(level.starting_code(), "# Paint a square\n");
    }

    #[test]
    fn each_map_gives_its_own_world() {
        let maps = maps(LEVEL);
        assert_eq!(maps.len(), 2);
        let sizes: Vec<_> = maps
            .iter()
            .map(|map| (map.world.width, map.world.height, map.world.cells.len()))
            .collect();
        assert_eq!(sizes, [(3, 2, 6), (5, 1, 5)]);
        assert_eq!(maps[1].world.robot.position, at(1, 0));
    }

    #[test]
    fn the_cells_to_paint_are_marked_once_with_the_passages_of_their_strokes() {
        let red = |at, passages: &[Direction]| PaintGoal {
            at,
            color: Color::Red,
            passages: passages.to_vec(),
        };
        assert_eq!(
            maps(LEVEL)[0].goals,
            Goals {
                robot_at: Some(at(0, 0)),
                // The square closes on its first cell, given once with its two passages.
                paints: vec![
                    red(at(0, 0), &[East, South]),
                    red(at(1, 0), &[South, West]),
                    red(at(1, 1), &[North, West]),
                    red(at(0, 1), &[North, East]),
                    PaintGoal {
                        at: at(2, 1),
                        color: Color::Blue,
                        passages: Vec::new(),
                    },
                ],
                items_at: Vec::new(),
            }
        );
    }

    #[test]
    fn the_cells_of_the_items_are_marked_in_the_order_of_the_file() {
        assert_eq!(
            maps(LEVEL)[1].goals,
            Goals {
                robot_at: None,
                paints: Vec::new(),
                items_at: vec![at(4, 0), at(3, 0)],
            }
        );
    }

    #[test]
    fn the_sandbox_gives_its_starting_code_and_its_world() {
        let sandbox = Sandbox::new(SANDBOX).expect("a valid sandbox file");
        assert_eq!(sandbox.starting_code(), "# Try anything here.\n");
        let core = level::Sandbox::from_toml(SANDBOX).expect("a valid sandbox file");
        let world = World::from(core.world());
        assert_eq!((world.width, world.height), (2, 1));
        assert_eq!(world.robot.direction, Direction::North);
    }
}
