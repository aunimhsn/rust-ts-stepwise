//! What the tests of several modules share.

use crate::color::Color;
use crate::span::{Position, Span};
use crate::world::{Cell, Direction, Map, Point, Robot, World};

/// The stretch of `line` from the column `start` included to the column `end` excluded.
pub fn span(line: u32, start: u32, end: u32) -> Span {
    Span::new(Position::new(line, start), Position::new(line, end))
}

/// A world drawn as rows of characters: `#` a wall, `.` a floor, `o` an item, `r` `y` `b` a
/// paint (without passages), and the robot `^` `>` `v` `<` facing north, east, south or west,
/// on a floor cell.
#[track_caller]
pub fn world(rows: &[&str]) -> World {
    let width = rows[0].chars().count();
    let height = rows.len();
    let mut map = Map::new(width, height).unwrap();
    let mut robot = None;
    for (y, row) in (0..).zip(rows) {
        for (x, character) in (0..).zip(row.chars()) {
            let point = Point::new(x, y);
            match character {
                '#' => map.set_wall(point).unwrap(),
                'o' => map.set_item(point).unwrap(),
                'r' => map.set_paint(point, Color::Red).unwrap(),
                'y' => map.set_paint(point, Color::Yellow).unwrap(),
                'b' => map.set_paint(point, Color::Blue).unwrap(),
                '^' => robot = Some(Robot::new(point, Direction::North)),
                '>' => robot = Some(Robot::new(point, Direction::East)),
                'v' => robot = Some(Robot::new(point, Direction::South)),
                '<' => robot = Some(Robot::new(point, Direction::West)),
                '.' => {}
                other => panic!("unknown character {other:?}"),
            }
        }
    }
    World::new(map, robot.expect("a robot")).unwrap()
}

/// The paints of the map, as rows of characters: `#` a wall, `.` an unpainted floor, `r` `y`
/// `b` a paint.
pub fn paints(world: &World) -> Vec<String> {
    let map = world.map();
    (0..map.height())
        .map(|y| {
            (0..map.width())
                .map(|x| match map.cell(Point::new(x, y)) {
                    Some(Cell::Floor(floor)) => match floor.paint() {
                        Some(Color::Red) => 'r',
                        Some(Color::Yellow) => 'y',
                        Some(Color::Blue) => 'b',
                        None => '.',
                    },
                    _ => '#',
                })
                .collect()
        })
        .collect()
}

/// A small generator of numbers, the same at every run, without a dependency.
pub struct Random(pub u64);

impl Random {
    /// The next number.
    pub fn next(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from(self.0 >> 33).unwrap()
    }

    /// One of `pieces`, which must not be empty.
    pub fn pick<'a>(&mut self, pieces: &[&'a str]) -> &'a str {
        pieces[self.next() % pieces.len()]
    }

    /// A program of one to twelve pieces picked among `pieces`, which must not be empty. The
    /// indentation follows the blocks (a piece that ends with `:` opens one), so that most
    /// programs have a right syntax. A piece is a line, or several lines that only make sense
    /// together, such as an `if` with its block and its `else`, written from the level of
    /// their first line.
    pub fn program(&mut self, pieces: &[&str]) -> String {
        let mut source = String::new();
        let mut level = 0;
        let mut opens = false;
        for _ in 0..=(self.next() % 12) {
            if opens {
                level += 1;
            } else {
                level -= self.next() % (level + 1);
            }
            let piece = self.pick(pieces);
            for line in piece.split('\n') {
                source.push_str(&" ".repeat(level * 4));
                source.push_str(line);
                source.push('\n');
            }
            opens = piece.ends_with(':');
        }
        source
    }
}
