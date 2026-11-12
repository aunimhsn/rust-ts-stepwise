//! What the tests of several modules share.

use crate::span::{Position, Span};

/// The stretch of `line` from the column `start` included to the column `end` excluded.
pub fn span(line: u32, start: u32, end: u32) -> Span {
    Span::new(Position::new(line, start), Position::new(line, end))
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
