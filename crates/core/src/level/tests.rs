//! The tests of the level files: what they hold, what they refuse, and the goals checked on a
//! world.

use super::{GoalNotMet, Level, LevelError, LevelErrorKind, Sandbox};
use crate::color::Color;
use crate::world::{BuildError, Cell, Direction, Point, World};

/// A level file with these maps, each the body of a `[[maps]]` table.
fn source(maps: &[&str]) -> String {
    let mut source = String::from("version = 1\nstarting_code = ''\n");
    for map in maps {
        source.push_str("\n[[maps]]\n");
        source.push_str(map);
    }
    source
}

/// The level of one map.
#[track_caller]
fn level(map: &str) -> Level {
    Level::from_toml(&source(&[map])).unwrap()
}

/// The error of a level of one map, which must be about that map.
#[track_caller]
fn map_error(map: &str) -> LevelErrorKind {
    let error = Level::from_toml(&source(&[map])).unwrap_err();
    assert_eq!(error.map, Some(1), "{error}");
    error.kind
}

/// The error of a level file, which must be about the file as a whole.
#[track_caller]
fn file_error(source: &str) -> LevelErrorKind {
    let error = Level::from_toml(source).unwrap_err();
    assert_eq!(error.map, None, "{error}");
    error.kind
}

/// A map of 4 × 2 floor cells, the robot at the top left facing east, the goal to stand at
/// the top right: the rest of a map, written after it, must not open another table.
const SMALL: &str = r#"
terrain = '''
....
....
'''
robot = { at = [0, 0], facing = "east" }
"#;

/// The goal of [`SMALL`].
const STAND_TOP_RIGHT: &str = "\n[maps.goals]\nrobot_at = [3, 0]\n";

/// [`SMALL`] with `extra` lines and the goal to stand at the top right.
fn small(extra: &str) -> String {
    format!("{SMALL}{extra}{STAND_TOP_RIGHT}")
}

/// [`SMALL`] with these goals.
fn small_with_goals(goals: &str) -> String {
    format!("{SMALL}\n[maps.goals]\n{goals}")
}

fn the_world(level: &Level) -> &World {
    level.maps()[0].world()
}

mod reading {
    use super::*;

    const EXAMPLE: &str = r#"
version = 1

starting_code = '''
# Paint a square of side 3
repeat 4:
    forward()
'''

[[maps]]
# Map 1: a blue stroke to paint over.
terrain = '''
#######
#.....#
#.....#
#.....#
#######
'''
robot = { at = [1, 1], facing = "east" }
items = [[5, 3]]
paints = [
    { color = "blue", path = [[1, 3], [2, 3]] },
]

[maps.goals]
robot_at = [5, 3]
paints = [
    { color = "red", path = [[1, 1], [2, 1], [3, 1], [3, 2], [3, 3], [2, 3], [1, 3], [1, 2], [1, 1]] },
]
nothing_else_painted = true
items_at = [[1, 2]]
carrying = false
"#;

    #[test]
    fn the_starting_code_is_kept_as_written() {
        let level = Level::from_toml(EXAMPLE).unwrap();
        assert_eq!(
            level.starting_code(),
            "# Paint a square of side 3\nrepeat 4:\n    forward()\n"
        );
    }

    #[test]
    fn the_terrain_gives_the_walls_and_the_floor() {
        let level = Level::from_toml(EXAMPLE).unwrap();
        let map = the_world(&level).map();
        assert_eq!((map.width(), map.height()), (7, 5));
        assert_eq!(map.cell(Point::new(0, 0)), Some(Cell::Wall));
        assert_eq!(map.cell(Point::new(6, 2)), Some(Cell::Wall));
        assert!(map.is_floor(Point::new(1, 1)));
        assert!(map.is_floor(Point::new(5, 3)));
    }

    #[test]
    fn the_robot_items_and_paints_are_placed() {
        let level = Level::from_toml(EXAMPLE).unwrap();
        let world = the_world(&level);
        let robot = world.robot();
        assert_eq!(robot.position(), Point::new(1, 1));
        assert_eq!(robot.direction(), Direction::East);
        assert!(!robot.is_carrying());
        let map = world.map();
        assert!(map.has_item(Point::new(5, 3)));
        assert_eq!(map.paint(Point::new(1, 3)), Some(Color::Blue));
        assert_eq!(map.paint(Point::new(2, 3)), Some(Color::Blue));
        assert!(map.passage(Point::new(1, 3), Direction::East));
        assert_eq!(map.paint(Point::new(3, 3)), None);
    }

    #[test]
    fn the_goals_are_read() {
        let level = Level::from_toml(EXAMPLE).unwrap();
        let goals = level.maps()[0].goals();
        assert_eq!(goals.robot_at(), Some(Point::new(5, 3)));
        assert_eq!(goals.paints().len(), 1);
        assert_eq!(goals.paints()[0].color(), Color::Red);
        assert_eq!(goals.paints()[0].path().len(), 9);
        assert!(goals.nothing_else_painted());
        assert_eq!(goals.items_at(), [Point::new(1, 2)]);
        assert_eq!(goals.carrying(), Some(false));
    }

    #[test]
    fn the_optional_fields_have_their_defaults() {
        let level = level(&small(""));
        let world = the_world(&level);
        assert!(!world.robot().is_carrying());
        assert!(world.map().points().all(|at| !world.map().has_item(at)));
        assert!(
            world
                .map()
                .points()
                .all(|at| world.map().paint(at).is_none())
        );
        let goals = level.maps()[0].goals();
        assert_eq!(goals.paints(), []);
        assert!(!goals.nothing_else_painted());
        assert_eq!(goals.items_at(), []);
        assert_eq!(goals.carrying(), None);
    }

    #[test]
    fn the_robot_may_start_carrying() {
        let level = level(
            &small_with_goals("carrying = false")
                .replace(r#"facing = "east""#, r#"facing = "west", carrying = true"#),
        );
        let robot = the_world(&level).robot();
        assert!(robot.is_carrying());
        assert_eq!(robot.direction(), Direction::West);
    }

    #[test]
    fn the_maps_are_variants_of_their_own_size_and_content() {
        let second = r#"
terrain = '''
......
'''
robot = { at = [0, 0], facing = "east" }
items = [[2, 0], [4, 0]]

[maps.goals]
items_at = [[5, 0], [3, 0]]
"#;
        let level = Level::from_toml(&source(&[&small(""), second])).unwrap();
        assert_eq!(level.maps().len(), 2);
        let map = level.maps()[1].world().map();
        assert_eq!((map.width(), map.height()), (6, 1));
        assert_eq!(
            level.maps()[1].goals().items_at(),
            [Point::new(5, 0), Point::new(3, 0)]
        );
    }

    #[test]
    fn a_stroke_painted_over_keeps_its_passages() {
        let level = level(&small(
            r#"paints = [
    { color = "red", path = [[0, 1], [1, 1], [2, 1]] },
    { color = "blue", path = [[2, 1], [3, 1]] },
]
"#,
        ));
        let map = the_world(&level).map();
        assert_eq!(map.paint(Point::new(1, 1)), Some(Color::Red));
        assert_eq!(map.paint(Point::new(2, 1)), Some(Color::Blue));
        assert!(map.passage(Point::new(1, 1), Direction::East));
        assert!(map.passage(Point::new(2, 1), Direction::East));
    }

    #[test]
    fn a_path_of_one_cell_paints_that_cell() {
        let level = level(&small(
            "paints = [{ color = \"yellow\", path = [[1, 0]] }]\n",
        ));
        let map = the_world(&level).map();
        assert_eq!(map.paint(Point::new(1, 0)), Some(Color::Yellow));
        assert!(!map.passage(Point::new(1, 0), Direction::East));
        assert!(!map.passage(Point::new(1, 0), Direction::West));
    }

    #[test]
    fn a_path_may_close_on_itself() {
        let level = level(&small_with_goals(
            "paints = [{ color = \"red\", path = [[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]] }]",
        ));
        let goals = level.maps()[0].goals();
        assert_eq!(goals.paints()[0].path().len(), 5);
    }

    #[test]
    fn the_cells_to_paint_are_given_once_in_the_order_of_the_strokes() {
        // The first stroke closes on its first cell, and the second comes back on two of them.
        let level = level(&small_with_goals(
            r#"paints = [
    { color = "red", path = [[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]] },
    { color = "red", path = [[2, 0], [1, 0], [1, 1], [2, 1]] },
    { color = "blue", path = [[3, 1]] },
]"#,
        ));
        let red = |x, y| (Point::new(x, y), Color::Red);
        assert_eq!(
            level.maps()[0].goals().paint_cells(),
            [
                red(0, 0),
                red(1, 0),
                red(1, 1),
                red(0, 1),
                red(2, 0),
                red(2, 1),
                (Point::new(3, 1), Color::Blue),
            ]
        );
    }

    #[test]
    fn the_passages_of_the_goals_are_those_their_strokes_cross() {
        let level = level(&small_with_goals(
            r#"paints = [
    { color = "red", path = [[0, 0], [1, 0], [1, 1]] },
    { color = "blue", path = [[2, 1], [3, 1]] },
    { color = "blue", path = [[3, 0]] },
]"#,
        ));
        let goals = level.maps()[0].goals();
        let passages = |x, y| {
            Direction::ALL
                .into_iter()
                .filter(|&direction| goals.passage(Point::new(x, y), direction))
                .collect::<Vec<_>>()
        };
        // A passage is crossed from both of its cells, whatever the way of the stroke.
        assert_eq!(passages(0, 0), [Direction::East]);
        assert_eq!(passages(1, 0), [Direction::South, Direction::West]);
        assert_eq!(passages(1, 1), [Direction::North]);
        assert_eq!(passages(2, 1), [Direction::East]);
        assert_eq!(passages(3, 1), [Direction::West]);
        // Two strokes next to each other don't join: [1, 1] and [2, 1], [3, 0] and [3, 1].
        assert_eq!(passages(3, 0), []);
        // A cell that no stroke paints, and the cells past the edges of the map.
        assert_eq!(passages(0, 1), []);
        assert!(!goals.passage(Point::new(0, 0), Direction::North));
        assert!(!goals.passage(Point::new(0, 0), Direction::West));
    }
}

mod refused {
    use super::*;

    #[test]
    fn another_version_is_refused_for_its_version() {
        let source = source(&[&small("")]).replace("version = 1", "version = 2");
        assert_eq!(file_error(&source), LevelErrorKind::Version(2));
        let with_new_field = format!("{source}\nnew_field = true\n");
        assert_eq!(file_error(&with_new_field), LevelErrorKind::Version(2));
    }

    #[test]
    fn a_file_without_version_is_refused() {
        let source = source(&[&small("")]).replace("version = 1\n", "");
        let LevelErrorKind::Syntax(message) = file_error(&source) else {
            panic!("a syntax error");
        };
        assert!(message.contains("version"), "{message}");
    }

    #[test]
    fn invalid_toml_is_a_syntax_error() {
        assert!(matches!(
            file_error("version = 1\nstarting_code = '"),
            LevelErrorKind::Syntax(_)
        ));
    }

    #[test]
    fn an_unknown_field_is_refused() {
        for (wrong, right, field) in [
            ("robot_at =", "robot_on =", "robot_on"),
            ("robot = { at", "robot = { position", "position"),
            ("terrain =", "ground =", "ground"),
        ] {
            let source = source(&[&small("")]).replace(wrong, right);
            let LevelErrorKind::Syntax(message) = file_error(&source) else {
                panic!("a syntax error for {field}");
            };
            assert!(message.contains("unknown field"), "{message}");
            assert!(message.contains(field), "{message}");
        }
    }

    #[test]
    fn an_unknown_color_or_direction_is_refused() {
        let paint = small("paints = [{ color = \"green\", path = [[0, 1]] }]\n");
        let LevelErrorKind::Syntax(message) = file_error(&source(&[&paint])) else {
            panic!("a syntax error");
        };
        assert!(
            message.contains("unknown color `green`, expected one of: red, yellow, blue"),
            "{message}"
        );
        let facing = small("").replace("\"east\"", "\"up\"");
        assert!(matches!(
            file_error(&source(&[&facing])),
            LevelErrorKind::Syntax(_)
        ));
    }

    #[test]
    fn a_coordinate_out_of_range_is_refused() {
        let below_zero = small("").replace("at = [0, 0]", "at = [-1, 0]");
        assert!(matches!(
            file_error(&source(&[&below_zero])),
            LevelErrorKind::Syntax(_)
        ));
    }

    #[test]
    fn a_level_has_one_to_three_maps() {
        assert_eq!(file_error(&source(&[])), LevelErrorKind::MapCount(0));
        let one = small("");
        assert_eq!(
            file_error(&source(&[&one, &one, &one, &one])),
            LevelErrorKind::MapCount(4)
        );
        assert!(Level::from_toml(&source(&[&one, &one, &one])).is_ok());
    }

    #[test]
    fn the_errors_of_a_map_give_its_number() {
        let wrong = small("").replace("....\n'''", "...\n'''");
        let error = Level::from_toml(&source(&[&small(""), &wrong])).unwrap_err();
        assert_eq!(error.map, Some(2));
        assert_eq!(
            error.to_string(),
            "map 2: row 1 of the terrain has 3 cells, while the first one has 4"
        );
    }

    #[test]
    fn the_terrain_fits_in_the_size_of_a_map() {
        let wide = small("").replace(
            "....\n....",
            &format!("{}\n{}", ".".repeat(17), ".".repeat(17)),
        );
        assert_eq!(
            map_error(&wide),
            LevelErrorKind::Map(BuildError::Size {
                width: 17,
                height: 2
            })
        );
        let empty = small("").replace("....\n....\n", "");
        assert_eq!(
            map_error(&empty),
            LevelErrorKind::Map(BuildError::Size {
                width: 0,
                height: 0
            })
        );
    }

    #[test]
    fn the_terrain_has_rows_of_the_same_length_and_known_characters() {
        let short = small("").replace("....\n'''", "..\n'''");
        assert_eq!(
            map_error(&short),
            LevelErrorKind::RowLength {
                y: 1,
                length: 2,
                expected: 4
            }
        );
        let unknown = small("").replace("....\n'''", "..o.\n'''");
        assert_eq!(
            map_error(&unknown),
            LevelErrorKind::UnknownCharacter {
                at: Point::new(2, 1),
                character: 'o'
            }
        );
        let trailing_line = small("").replace("....\n'''", "....\n\n'''");
        assert!(matches!(
            map_error(&trailing_line),
            LevelErrorKind::RowLength {
                y: 2,
                length: 0,
                ..
            }
        ));
    }

    #[test]
    fn nothing_is_placed_on_a_wall_or_outside_the_map() {
        let walled = |extra: &str| small(extra).replace("....\n'''", "#...\n'''");
        assert_eq!(
            map_error(&walled("items = [[0, 1]]\n")),
            LevelErrorKind::Map(BuildError::OnWall(Point::new(0, 1)))
        );
        assert_eq!(
            map_error(&walled("paints = [{ color = \"red\", path = [[0, 1]] }]\n")),
            LevelErrorKind::Map(BuildError::OnWall(Point::new(0, 1)))
        );
        assert_eq!(
            map_error(&small("items = [[4, 0]]\n")),
            LevelErrorKind::Map(BuildError::OutsideMap(Point::new(4, 0)))
        );
        let robot = small("").replace("at = [0, 0]", "at = [0, 2]");
        assert_eq!(
            map_error(&robot),
            LevelErrorKind::Map(BuildError::OutsideMap(Point::new(0, 2)))
        );
        let robot_on_wall = small("").replace("....\n....", "#...\n....");
        assert_eq!(
            map_error(&robot_on_wall),
            LevelErrorKind::Map(BuildError::OnWall(Point::new(0, 0)))
        );
    }

    #[test]
    fn two_items_on_one_cell_are_refused() {
        assert_eq!(
            map_error(&small("items = [[1, 1], [1, 1]]\n")),
            LevelErrorKind::DuplicateItem(Point::new(1, 1))
        );
    }

    #[test]
    fn a_path_is_made_of_cells_next_to_each_other() {
        assert_eq!(
            map_error(&small(
                "paints = [{ color = \"red\", path = [[0, 1], [2, 1]] }]\n"
            )),
            LevelErrorKind::NotNeighbors(Point::new(0, 1), Point::new(2, 1))
        );
        assert_eq!(
            map_error(&small(
                "paints = [{ color = \"red\", path = [[0, 1], [1, 0]] }]\n"
            )),
            LevelErrorKind::NotNeighbors(Point::new(0, 1), Point::new(1, 0))
        );
        assert_eq!(
            map_error(&small("paints = [{ color = \"red\", path = [] }]\n")),
            LevelErrorKind::EmptyPath
        );
    }
}

mod goals_refused {
    use super::*;

    #[test]
    fn a_map_of_a_level_has_a_goal() {
        assert_eq!(map_error(SMALL), LevelErrorKind::NoGoal);
        assert_eq!(map_error(&small_with_goals("")), LevelErrorKind::NoGoal);
        assert_eq!(
            map_error(&small_with_goals(
                "nothing_else_painted = false\npaints = []"
            )),
            LevelErrorKind::NoGoal
        );
    }

    #[test]
    fn goals_all_met_at_the_start_are_refused() {
        for goals in [
            "robot_at = [0, 0]",
            "carrying = false",
            "nothing_else_painted = true",
        ] {
            assert_eq!(
                map_error(&small_with_goals(goals)),
                LevelErrorKind::GoalsMetAtStart,
                "{goals}"
            );
        }
        let painted = format!(
            "{SMALL}paints = [{{ color = \"red\", path = [[1, 1]] }}]\n\n[maps.goals]\n\
             paints = [{{ color = \"red\", path = [[1, 1]] }}]\n"
        );
        assert_eq!(map_error(&painted), LevelErrorKind::GoalsMetAtStart);
    }

    #[test]
    fn some_goals_may_be_met_at_the_start() {
        let goals = "robot_at = [3, 0]\ncarrying = false\nnothing_else_painted = true";
        assert!(Level::from_toml(&source(&[&small_with_goals(goals)])).is_ok());
    }

    #[test]
    fn nothing_else_painted_alone_is_a_goal_when_a_cell_is_painted() {
        let erase = format!(
            "{SMALL}paints = [{{ color = \"red\", path = [[1, 1]] }}]\n\n[maps.goals]\n\
             nothing_else_painted = true\n"
        );
        assert!(Level::from_toml(&source(&[&erase])).is_ok());
    }

    #[test]
    fn the_goals_need_no_more_items_than_the_map_has() {
        let goals = "items_at = [[3, 0], [3, 1]]";
        assert_eq!(
            map_error(&format!(
                "{SMALL}items = [[1, 0]]\n\n[maps.goals]\n{goals}\n"
            )),
            LevelErrorKind::NotEnoughItems {
                needed: 2,
                available: 1
            }
        );
        assert_eq!(
            map_error(&small_with_goals("carrying = true")),
            LevelErrorKind::NotEnoughItems {
                needed: 1,
                available: 0
            }
        );
        let carried = format!("{SMALL}items = [[1, 0]]\n\n[maps.goals]\n{goals}\n")
            .replace(r#"facing = "east""#, r#"facing = "east", carrying = true"#);
        assert!(Level::from_toml(&source(&[&carried])).is_ok());
    }

    #[test]
    fn a_goal_lies_on_a_floor_cell_of_the_map() {
        let walled = |goals: &str| small_with_goals(goals).replace("....\n'''", "...#\n'''");
        assert_eq!(
            map_error(&walled("robot_at = [3, 1]")),
            LevelErrorKind::Goal(BuildError::OnWall(Point::new(3, 1)))
        );
        assert_eq!(
            map_error(&walled("items_at = [[3, 1]]")),
            LevelErrorKind::Goal(BuildError::OnWall(Point::new(3, 1)))
        );
        assert_eq!(
            map_error(&walled(
                "paints = [{ color = \"red\", path = [[2, 1], [3, 1]] }]"
            )),
            LevelErrorKind::Goal(BuildError::OnWall(Point::new(3, 1)))
        );
        assert_eq!(
            map_error(&small_with_goals("robot_at = [0, 5]")),
            LevelErrorKind::Goal(BuildError::OutsideMap(Point::new(0, 5)))
        );
    }

    #[test]
    fn the_strokes_of_the_goals_are_checked_as_strokes() {
        assert_eq!(
            map_error(&small_with_goals(
                "paints = [{ color = \"red\", path = [[0, 0], [1, 1]] }]"
            )),
            LevelErrorKind::NotNeighbors(Point::new(0, 0), Point::new(1, 1))
        );
    }

    #[test]
    fn a_cell_has_one_color_in_the_goals() {
        assert_eq!(
            map_error(&small_with_goals(
                "paints = [\n    { color = \"red\", path = [[0, 0], [1, 0]] },\n    \
                 { color = \"blue\", path = [[1, 0], [1, 1]] },\n]"
            )),
            LevelErrorKind::ConflictingPaintGoal(Point::new(1, 0))
        );
        assert!(
            Level::from_toml(&source(&[&small_with_goals(
                "paints = [\n    { color = \"red\", path = [[0, 0], [1, 0]] },\n    \
                 { color = \"red\", path = [[1, 0], [1, 1]] },\n]"
            )]))
            .is_ok()
        );
    }

    #[test]
    fn a_cell_is_given_once_for_an_item() {
        assert_eq!(
            map_error(&format!(
                "{SMALL}items = [[1, 0], [2, 0]]\n\n[maps.goals]\nitems_at = [[3, 1], [3, 1]]\n"
            )),
            LevelErrorKind::DuplicateItemGoal(Point::new(3, 1))
        );
    }

    #[test]
    fn standing_and_an_item_may_share_a_cell() {
        let goals = format!(
            "{SMALL}items = [[1, 0]]\n\n[maps.goals]\nrobot_at = [3, 1]\nitems_at = [[3, 1]]\n"
        );
        assert!(Level::from_toml(&source(&[&goals])).is_ok());
    }
}

mod checking {
    use super::*;

    /// The goals of the one map of `goals` on [`SMALL`] with `extra`, checked on the world
    /// that `act` leaves.
    fn not_met(extra: &str, goals: &str, act: impl FnOnce(&mut World)) -> Vec<GoalNotMet> {
        let level = level(&format!("{SMALL}{extra}\n[maps.goals]\n{goals}\n"));
        let map = &level.maps()[0];
        let mut world = map.world().clone();
        act(&mut world);
        map.goals().check(&world)
    }

    #[test]
    fn where_to_stand() {
        assert_eq!(
            not_met("", "robot_at = [2, 0]", |world| world.forward().unwrap()),
            [GoalNotMet::RobotNotAt(Point::new(2, 0))]
        );
        assert_eq!(
            not_met("", "robot_at = [2, 0]", |world| {
                world.forward().unwrap();
                world.forward().unwrap();
            }),
            []
        );
    }

    #[test]
    fn the_paints_look_at_the_cells_only() {
        let goals = "paints = [{ color = \"red\", path = [[0, 0], [1, 0], [2, 0]] }]";
        assert_eq!(
            not_met("", goals, |world| {
                world.start_painting(Color::Red);
                world.forward().unwrap();
                world.start_painting(Color::Blue);
            }),
            [
                GoalNotMet::PaintMissing {
                    at: Point::new(1, 0),
                    color: Color::Red
                },
                GoalNotMet::PaintMissing {
                    at: Point::new(2, 0),
                    color: Color::Red
                },
            ]
        );
        let painted_cell_by_cell = |world: &mut World| {
            for _ in 0..2 {
                world.start_painting(Color::Red);
                world.stop_painting();
                world.forward().unwrap();
            }
            world.start_painting(Color::Red);
        };
        assert_eq!(not_met("", goals, painted_cell_by_cell), []);
    }

    #[test]
    fn a_cell_given_twice_is_checked_once() {
        let goals =
            "paints = [{ color = \"blue\", path = [[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]] }]";
        assert_eq!(
            not_met("", goals, |_| {}),
            [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(0, 1)
            ]
            .map(|at| GoalNotMet::PaintMissing {
                at,
                color: Color::Blue
            })
        );
    }

    #[test]
    fn nothing_else_painted_flags_each_cell_in_excess() {
        let goals = "paints = [{ color = \"red\", path = [[1, 0]] }]\nnothing_else_painted = true";
        assert_eq!(
            not_met(
                "paints = [{ color = \"yellow\", path = [[3, 1], [3, 0]] }]\n",
                goals,
                |world| {
                    world.forward().unwrap();
                    world.start_painting(Color::Red);
                    world.forward().unwrap();
                }
            ),
            [
                GoalNotMet::PaintedElsewhere(Point::new(2, 0)),
                GoalNotMet::PaintedElsewhere(Point::new(3, 0)),
                GoalNotMet::PaintedElsewhere(Point::new(3, 1)),
            ]
        );
    }

    #[test]
    fn the_items_and_what_the_robot_carries() {
        let extra = "items = [[1, 0]]\n";
        let goals = "items_at = [[2, 0]]\ncarrying = false";
        assert_eq!(
            not_met(extra, goals, |world| {
                world.forward().unwrap();
                world.pick_up().unwrap();
            }),
            [
                GoalNotMet::ItemMissing(Point::new(2, 0)),
                GoalNotMet::Carrying
            ]
        );
        assert_eq!(
            not_met(extra, goals, |world| {
                world.forward().unwrap();
                world.pick_up().unwrap();
                world.forward().unwrap();
                world.drop_item().unwrap();
            }),
            []
        );
        assert_eq!(
            not_met(extra, "carrying = true", |_| {}),
            [GoalNotMet::NotCarrying]
        );
    }

    #[test]
    fn every_goal_not_met_is_given_in_the_order_of_the_goals() {
        let extra = "items = [[1, 0], [2, 0]]\npaints = [{ color = \"blue\", path = [[0, 1]] }]\n";
        let goals = "robot_at = [3, 1]\npaints = [{ color = \"red\", path = [[2, 1]] }]\n\
                     nothing_else_painted = true\nitems_at = [[3, 0]]\ncarrying = true";
        assert_eq!(
            not_met(extra, goals, |_| {}),
            [
                GoalNotMet::RobotNotAt(Point::new(3, 1)),
                GoalNotMet::PaintMissing {
                    at: Point::new(2, 1),
                    color: Color::Red
                },
                GoalNotMet::PaintedElsewhere(Point::new(0, 1)),
                GoalNotMet::ItemMissing(Point::new(3, 0)),
                GoalNotMet::NotCarrying,
            ]
        );
    }

    #[test]
    fn the_cell_of_a_goal_not_met_is_where_its_flag_goes() {
        let at = Point::new(1, 2);
        assert_eq!(GoalNotMet::RobotNotAt(at).cell(), Some(at));
        assert_eq!(
            GoalNotMet::PaintMissing {
                at,
                color: Color::Red
            }
            .cell(),
            Some(at)
        );
        assert_eq!(GoalNotMet::PaintedElsewhere(at).cell(), Some(at));
        assert_eq!(GoalNotMet::ItemMissing(at).cell(), Some(at));
        assert_eq!(GoalNotMet::NotCarrying.cell(), None);
        assert_eq!(GoalNotMet::Carrying.cell(), None);
    }
}

mod sandbox {
    use super::*;

    const SANDBOX: &str = r#"
version = 1
starting_code = '''
# Try anything here.
'''

[map]
terrain = '''
...
.#.
'''
robot = { at = [0, 0], facing = "south" }
items = [[2, 1]]
paints = [{ color = "yellow", path = [[0, 1], [0, 0], [1, 0]] }]
"#;

    #[test]
    fn the_sandbox_has_one_map_without_goals() {
        let sandbox = Sandbox::from_toml(SANDBOX).unwrap();
        assert_eq!(sandbox.starting_code(), "# Try anything here.\n");
        let map = sandbox.world().map();
        assert_eq!(map.cell(Point::new(1, 1)), Some(Cell::Wall));
        assert!(map.has_item(Point::new(2, 1)));
        assert!(map.passage(Point::new(0, 0), Direction::South));
        assert_eq!(sandbox.world().robot().direction(), Direction::South);
    }

    #[test]
    fn the_sandbox_refuses_goals() {
        let with_goals = format!("{SANDBOX}\n[map.goals]\nrobot_at = [2, 0]\n");
        assert_eq!(
            Sandbox::from_toml(&with_goals).unwrap_err(),
            LevelError {
                map: None,
                kind: LevelErrorKind::GoalsInSandbox
            }
        );
    }

    #[test]
    fn the_sandbox_refuses_the_maps_of_a_level() {
        let as_level = SANDBOX.replace("[map]", "[[maps]]");
        assert!(matches!(
            Sandbox::from_toml(&as_level).unwrap_err().kind,
            LevelErrorKind::Syntax(_)
        ));
        assert!(matches!(
            Level::from_toml(SANDBOX).unwrap_err().kind,
            LevelErrorKind::Syntax(_)
        ));
    }

    #[test]
    fn the_errors_of_its_map_have_no_number() {
        let wrong = SANDBOX.replace("items = [[2, 1]]", "items = [[1, 1]]");
        assert_eq!(
            Sandbox::from_toml(&wrong).unwrap_err(),
            LevelError {
                map: None,
                kind: LevelErrorKind::Map(BuildError::OnWall(Point::new(1, 1)))
            }
        );
    }
}
