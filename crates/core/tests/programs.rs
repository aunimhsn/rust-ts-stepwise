//! Whole programs run on the maps of levels, through the public API alone and in the order
//! the website will follow: the level file read, the program checked, run one step at a time
//! to its end, then the goals of the map checked on the world the program leaves.
//!
//! The levels of the website come with the steps of their chapters: the ones of
//! `trial-levels/` are for these tests only.

use stepwise_core::checker::check;
use stepwise_core::color::Color;
use stepwise_core::interpreter::{Machine, RuntimeErrorKind, State};
use stepwise_core::level::{GoalNotMet, Level};
use stepwise_core::parser::parse;
use stepwise_core::world::{Point, RobotError};

const PAINT_A_SQUARE: &str = include_str!("trial-levels/paint-a-square.toml");
const CARRY_THE_ITEM: &str = include_str!("trial-levels/carry-the-item.toml");
const TURN_THE_CORNER: &str = include_str!("trial-levels/turn-the-corner.toml");

/// The most steps a program of these tests runs: they all end well before.
const MAX_STEPS: usize = 1000;

/// How a program ends on a map.
#[derive(Debug, PartialEq, Eq)]
enum End {
    /// It ended, every goal met: the map is passed.
    Passed,
    /// It ended with these goals not met.
    GoalsNotMet(Vec<GoalNotMet>),
    /// It stopped on this error, on this line: the goals aren't checked.
    Stopped(RuntimeErrorKind, u32),
}

/// Runs `source` on each map of `level`, a level file, and says how it ends on each.
#[track_caller]
fn run(level: &str, source: &str) -> Vec<End> {
    let level = Level::from_toml(level).expect("a valid level file");
    let program = parse(source).expect("a program without syntax error");
    assert_eq!(check(&program), [], "in {source:?}");
    let ends = level.maps().iter().map(|map| {
        // Each map starts from its own world, which the level keeps as it is.
        let mut machine = Machine::new(&program, map.world().clone());
        for _ in 0..MAX_STEPS {
            if !matches!(machine.state(), State::Running { .. }) {
                break;
            }
            machine.step();
        }
        match machine.state() {
            State::Ended => {
                let not_met = map.goals().check(machine.world());
                if not_met.is_empty() {
                    End::Passed
                } else {
                    End::GoalsNotMet(not_met)
                }
            }
            State::Failed(error) => End::Stopped(error.kind.clone(), error.span.start.line),
            State::Running { .. } => panic!("{source:?} still runs after {MAX_STEPS} steps"),
        }
    });
    ends.collect()
}

fn stopped(error: RobotError, line: u32) -> End {
    End::Stopped(RuntimeErrorKind::Robot(error), line)
}

fn red_missing(x: u8, y: u8) -> GoalNotMet {
    GoalNotMet::PaintMissing {
        at: Point::new(x, y),
        color: Color::Red,
    }
}

#[test]
fn the_starting_code_of_the_trial_levels_is_checked() {
    for source in [PAINT_A_SQUARE, CARRY_THE_ITEM, TURN_THE_CORNER] {
        let level = Level::from_toml(source).expect("a valid level file");
        let program = parse(level.starting_code()).expect("a starting code without error");
        assert_eq!(check(&program), []);
        assert_eq!(level.maps().len(), 2);
    }
}

// Paint: the brush, the functions of the program, the goals of paint.

/// The example that opens *The language* in `specs.md`, as it is written there.
const SQUARE_OF_THE_SPECIFICATIONS: &str = "\
# Paint a square of side 3
function paint_side(length):
    start_painting(red)
    repeat length:
        forward()
    stop_painting()

let sides = 0
while sides < 4:
    paint_side(3)
    turn_right()
    sides = sides + 1

if front_is_clear():
    forward()
else if left_is_clear():
    turn_left()
else:
    print(\"I am stuck\")
";

#[test]
fn the_example_of_the_specifications_paints_its_square_on_each_map() {
    assert_eq!(
        run(PAINT_A_SQUARE, SQUARE_OF_THE_SPECIFICATIONS),
        [End::Passed, End::Passed]
    );
}

#[test]
fn a_square_with_a_side_missing_leaves_its_cells_as_goals_not_met() {
    let source = "\
function paint_side(length):
    start_painting(red)
    repeat length:
        forward()
    stop_painting()

repeat 3:
    paint_side(3)
    turn_right()
";
    // The corners of the fourth side are painted by the sides next to it.
    assert_eq!(
        run(PAINT_A_SQUARE, source),
        [
            End::GoalsNotMet(vec![red_missing(1, 3), red_missing(1, 2)]),
            End::GoalsNotMet(vec![red_missing(2, 0), red_missing(3, 0)]),
        ]
    );
}

#[test]
fn a_square_in_another_color_meets_no_goal_of_paint() {
    let source = SQUARE_OF_THE_SPECIFICATIONS.replace("red", "blue");
    let ends = run(PAINT_A_SQUARE, &source);
    for end in &ends {
        let End::GoalsNotMet(not_met) = end else {
            panic!("the goals should not be met: {end:?}");
        };
        // The twelve cells of the square, each painted in blue.
        assert_eq!(not_met.len(), 12, "{not_met:?}");
        assert!(
            not_met
                .iter()
                .all(|goal| matches!(goal, GoalNotMet::PaintMissing { .. }))
        );
    }
    assert_eq!(ends.len(), 2);
}

#[test]
fn a_side_too_long_bumps_into_the_edge_and_the_goals_are_not_checked() {
    let source = SQUARE_OF_THE_SPECIFICATIONS.replace("paint_side(3)", "paint_side(4)");
    // The line 5 is the `forward()` of the function.
    assert_eq!(
        run(PAINT_A_SQUARE, &source),
        [stopped(RobotError::Bump, 5), stopped(RobotError::Bump, 5)]
    );
}

// Carry: the items, the loops on a sensor, the goals of items.

#[test]
fn a_program_that_looks_for_the_item_brings_it_to_its_slot_on_each_map() {
    let source = "\
while not item_here():
    forward()
pick_up()
while front_is_clear():
    forward()
drop()
";
    assert_eq!(run(CARRY_THE_ITEM, source), [End::Passed, End::Passed]);
}

#[test]
fn a_program_that_counts_the_cells_of_one_map_does_not_pass_the_other() {
    let source = "\
repeat 2:
    forward()
pick_up()
repeat 4:
    forward()
drop()
";
    assert_eq!(
        run(CARRY_THE_ITEM, source),
        [End::Passed, stopped(RobotError::NoItemHere, 3)]
    );
}

#[test]
fn an_item_that_is_not_dropped_leaves_two_goals_not_met() {
    let source = "\
while not item_here():
    forward()
pick_up()
while front_is_clear():
    forward()
";
    let not_met = || {
        End::GoalsNotMet(vec![
            GoalNotMet::ItemMissing(Point::new(7, 1)),
            GoalNotMet::Carrying,
        ])
    };
    assert_eq!(run(CARRY_THE_ITEM, source), [not_met(), not_met()]);
}

// Decide: the sensors, `if` and `else`, the goal of where to stand.

#[test]
fn a_program_that_senses_the_walls_follows_the_corridor_on_each_map() {
    let source = "\
while front_is_clear():
    forward()
if left_is_clear():
    turn_left()
else:
    turn_right()
while front_is_clear():
    forward()
";
    assert_eq!(run(TURN_THE_CORNER, source), [End::Passed, End::Passed]);
}

#[test]
fn a_program_that_always_turns_right_stays_at_the_corner_of_the_other_map() {
    let source = "\
while front_is_clear():
    forward()
turn_right()
while front_is_clear():
    forward()
";
    let at_the_corner = End::GoalsNotMet(vec![GoalNotMet::RobotNotAt(Point::new(3, 0))]);
    assert_eq!(run(TURN_THE_CORNER, source), [End::Passed, at_the_corner]);
}

#[test]
fn an_empty_program_passes_no_map() {
    for level in [PAINT_A_SQUARE, CARRY_THE_ITEM, TURN_THE_CORNER] {
        for end in run(level, "# Nothing yet\n") {
            assert!(matches!(end, End::GoalsNotMet(_)), "{end:?}");
        }
    }
}
