//! Positions in the source of a program, shared by the tokens, the syntax tree and the errors.

/// A place in the source, before a character.
///
/// The line and the column both start at 1. The column counts characters (Unicode scalar
/// values), as a learner counts them, not bytes: in `print("côté")`, the closing quote is in
/// column 12. The bridge turns it into the offset the editor uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// The line, from 1.
    pub line: u32,
    /// The column, from 1, in characters.
    pub column: u32,
}

impl Position {
    /// The position at `line` and `column`, both from 1.
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// A stretch of the source, from `start` included to `end` excluded, on one line or more.
///
/// A span is empty when `start` and `end` are equal: it then marks a place between two
/// characters, such as the end of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// The first character of the stretch.
    pub start: Position,
    /// The position just after its last character.
    pub end: Position,
}

impl Span {
    /// The stretch from `start` included to `end` excluded.
    #[must_use]
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// The empty stretch at `position`.
    #[must_use]
    pub const fn at(position: Position) -> Self {
        Self {
            start: position,
            end: position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Span};

    #[test]
    fn positions_are_ordered_by_line_then_column() {
        assert!(Position::new(1, 9) < Position::new(2, 1));
        assert!(Position::new(2, 1) < Position::new(2, 2));
        assert_eq!(Position::new(3, 4), Position::new(3, 4));
    }

    #[test]
    fn a_span_at_a_position_is_empty() {
        let span = Span::at(Position::new(2, 5));
        assert_eq!(span.start, span.end);
        assert_eq!(span, Span::new(Position::new(2, 5), Position::new(2, 5)));
    }
}
