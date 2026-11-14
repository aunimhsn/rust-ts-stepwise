//! The places of a source as JavaScript counts them in its text.
//!
//! The core gives a line and a column, both from 1, the column in characters, as a learner
//! counts them. JavaScript indexes a text from its start, in UTF-16 code units, where an emoji
//! takes two. This module is the one place where the positions change (see Positions in
//! `docs/technical-choices.md`).
//!
//! The offsets are those of the very text that was checked. They are also those of the
//! editor when that text is the one of its document, where every line break is one `\n`: the
//! editor counts one unit for a line break, whatever the text it was given wrote there.

use serde::Serialize;
use stepwise_core::span::{self, Position};
use tsify::Tsify;

/// A stretch of the source, for the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Tsify)]
pub struct Span {
    /// The line of its start, from 1: what the console and the status bar write.
    pub line: u32,
    /// The offset of its start in the text that was checked, in UTF-16 code units, as
    /// JavaScript indexes it. The editor counts the same when that text is its document.
    pub from: u32,
    /// The offset just after its end. It equals `from` for a place between two characters,
    /// such as the end of a line where something is missing.
    pub to: u32,
}

/// The lines of a source, to turn the positions the core gave for it into offsets. It keeps
/// no text, so it outlives the source it was built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offsets {
    lines: Vec<Line>,
    /// The offset of the end of the source.
    end: u32,
}

/// A line of the source, as the lexer cuts them: without its line break.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    /// The offset of its first character.
    start: u32,
    /// The offset after its last character, before its line break.
    end: u32,
    /// The columns of its characters that take two units, in order: none in most lines.
    wide: Vec<u32>,
}

impl Offsets {
    /// The offsets of `source`, whose lines are cut as the lexer cuts them: a line break is
    /// `\n` or `\r\n`, and counts for one or two units.
    #[must_use]
    pub fn new(source: &str) -> Self {
        let mut lines = Vec::new();
        let mut start = 0_u32;
        for whole in source.split('\n') {
            // The line break: its `\n`, and its `\r` when the lexer takes one off the line.
            let (text, line_break) = match whole.strip_suffix('\r') {
                Some(text) => (text, 2),
                None => (whole, 1),
            };
            let mut end = start;
            let mut wide = Vec::new();
            for (column, character) in (1_u32..).zip(text.chars()) {
                let units = if character.len_utf16() > 1 {
                    wide.push(column);
                    2
                } else {
                    1
                };
                end = end.saturating_add(units);
            }
            lines.push(Line { start, end, wide });
            start = end.saturating_add(line_break);
        }
        let end = u32::try_from(source.encode_utf16().count()).unwrap_or(u32::MAX);
        Self { lines, end }
    }

    /// The offset of `position`.
    ///
    /// Every position the core gives for the source is in it. A column past the end of its
    /// line would give the end of that line, and a line the source doesn't have the end of the
    /// source, rather than an offset the editor would refuse.
    #[must_use]
    pub fn offset(&self, position: Position) -> u32 {
        let line = position
            .line
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| self.lines.get(index));
        let Some(line) = line else {
            return self.end;
        };
        // Each character before the column is one unit, and one more when it is wide.
        let characters = position.column.saturating_sub(1);
        let wide = line
            .wide
            .iter()
            .take_while(|&&column| column < position.column)
            .count();
        line.start
            .saturating_add(characters)
            .saturating_add(u32::try_from(wide).unwrap_or(u32::MAX))
            .min(line.end)
    }

    /// The stretch `span` of the core, for the interface.
    #[must_use]
    pub fn span(&self, span: span::Span) -> Span {
        Span {
            line: span.start.line,
            from: self.offset(span.start),
            to: self.offset(span.end),
        }
    }
}

#[cfg(test)]
mod tests {
    use stepwise_core::lexer::{TokenKind, tokenize};
    use stepwise_core::span::{self, Position};

    use super::{Offsets, Span};

    fn core_span(start: (u32, u32), end: (u32, u32)) -> span::Span {
        span::Span::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
    }

    /// What JavaScript would cut out of `source` with the offsets of `span`: the source in
    /// UTF-16 code units, as it indexes a string.
    #[track_caller]
    fn cut(source: &str, span: Span) -> String {
        let units: Vec<u16> = source.encode_utf16().collect();
        let (from, to) = (span.from as usize, span.to as usize);
        String::from_utf16(&units[from..to]).expect("offsets between two characters")
    }

    /// The stretch of `source` from `start` to `end`, each a line and a column, cut as
    /// JavaScript would.
    #[track_caller]
    fn text(source: &str, start: (u32, u32), end: (u32, u32)) -> String {
        cut(source, Offsets::new(source).span(core_span(start, end)))
    }

    #[test]
    fn a_span_keeps_its_line_and_becomes_two_offsets() {
        let source = "let sides = 4\nrepeat sides:\n    forward()\n";
        let span = Offsets::new(source).span(core_span((3, 5), (3, 14)));
        assert_eq!(
            span,
            Span {
                line: 3,
                from: 32,
                to: 41
            }
        );
        assert_eq!(cut(source, span), "forward()");
    }

    #[test]
    fn an_accent_is_one_unit_as_it_is_one_character() {
        let source = "let côté = 3\nprint(côté)\n";
        assert_eq!(text(source, (1, 5), (1, 9)), "côté");
        assert_eq!(text(source, (2, 7), (2, 11)), "côté");
    }

    #[test]
    fn an_emoji_is_two_units_for_one_character() {
        let source = "print(\"🤖\") + sidse\nprint(\"🤖🤖\", sidse)\n";
        let offsets = Offsets::new(source);
        // The emoji is the column 8 of the line 1, and `sidse` starts at the column 14.
        assert_eq!(offsets.offset(Position::new(1, 8)), 7);
        assert_eq!(offsets.offset(Position::new(1, 9)), 9);
        assert_eq!(text(source, (1, 8), (1, 9)), "🤖");
        assert_eq!(text(source, (1, 14), (1, 19)), "sidse");
        // The next line starts after every unit of the line above, and counts its own.
        assert_eq!(text(source, (2, 1), (2, 6)), "print");
        assert_eq!(text(source, (2, 8), (2, 10)), "🤖🤖");
        assert_eq!(text(source, (2, 13), (2, 18)), "sidse");
    }

    #[test]
    fn a_line_break_with_a_carriage_return_counts_for_two_units() {
        let source = "if x:\r\n    côté = \"🤖\"\r\nb\r\n";
        assert_eq!(text(source, (1, 1), (1, 6)), "if x:");
        assert_eq!(text(source, (2, 5), (2, 9)), "côté");
        assert_eq!(text(source, (3, 1), (3, 2)), "b");
        // The end of a line is before its line break, whatever its length.
        let offsets = Offsets::new(source);
        assert_eq!(offsets.offset(Position::new(1, 6)), 5);
        assert_eq!(offsets.offset(Position::new(2, 1)), 7);
    }

    #[test]
    fn a_span_goes_over_several_lines() {
        let source = "while true:\n    # 🤖\n    forward()\n";
        assert_eq!(
            text(source, (1, 1), (3, 14)),
            "while true:\n    # 🤖\n    forward()"
        );
        // Its line is the one where it starts.
        let span = Offsets::new(source).span(core_span((2, 5), (3, 14)));
        assert_eq!(span.line, 2);
    }

    #[test]
    fn an_empty_span_is_a_place_between_two_characters() {
        let source = "forward(\nlet sides =";
        let offsets = Offsets::new(source);
        // The end of the first line, and the end of the source, which has no line break.
        for (place, offset) in [((1, 9), 8), ((2, 12), 20)] {
            let span = offsets.span(core_span(place, place));
            assert_eq!((span.from, span.to), (offset, offset));
        }
    }

    #[test]
    fn the_end_of_the_source_is_the_place_the_lexer_gives_it() {
        // After a last line break, the end is the start of a line that has no character.
        assert_eq!(Offsets::new("").offset(Position::new(1, 1)), 0);
        assert_eq!(Offsets::new("ab").offset(Position::new(1, 3)), 2);
        assert_eq!(Offsets::new("ab\n").offset(Position::new(2, 1)), 3);
        assert_eq!(Offsets::new("🤖\r\n").offset(Position::new(2, 1)), 4);
    }

    #[test]
    fn every_position_of_a_source_is_the_length_of_what_is_before_it() {
        let source = "let côté = \"🤖 ok 🤖\"\r\n\n# 𝒳 and é\n    print(côté)\r\nend 🤖";
        let offsets = Offsets::new(source);
        let mut before = String::new();
        let (mut line, mut column) = (1, 1);
        for character in source.chars() {
            let expected = u32::try_from(before.encode_utf16().count()).unwrap();
            // Between the `\r` and the `\n` of a line break, there is no position: the line
            // ended before the `\r`.
            if !(character == '\n' && before.ends_with('\r')) {
                assert_eq!(
                    offsets.offset(Position::new(line, column)),
                    expected,
                    "at {line}:{column}, after {before:?}"
                );
            }
            before.push(character);
            if character == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        let end = u32::try_from(source.encode_utf16().count()).unwrap();
        assert_eq!(offsets.offset(Position::new(line, column)), end);
    }

    /// The lines are cut here as the lexer cuts them, a rule written in both crates: the
    /// positions of this test come from the lexer itself.
    #[test]
    fn every_name_the_lexer_reads_is_cut_where_it_is_written() {
        let source = "let côté = \"🤖\"\r\nif côté:\r\n    print(côté, sidse)\n# 🤖\nforward()";
        let offsets = Offsets::new(source);
        let mut names = Vec::new();
        for token in tokenize(source).unwrap().tokens {
            if let TokenKind::Name(name) = token.kind {
                assert_eq!(cut(source, offsets.span(token.span)), name);
                names.push(name);
            }
        }
        assert_eq!(names, ["côté", "côté", "print", "côté", "sidse", "forward"]);
    }

    #[test]
    fn a_place_the_source_does_not_have_stays_inside_it() {
        let source = "ab 🤖\ncd\n";
        let offsets = Offsets::new(source);
        // Past the end of a line: the end of that line, before its line break, whatever it is.
        assert_eq!(offsets.offset(Position::new(1, 99)), 5);
        assert_eq!(Offsets::new("ab\r\ncd\r\n").offset(Position::new(1, 99)), 2);
        assert_eq!(offsets.offset(Position::new(2, u32::MAX)), 8);
        // A line that isn't there, and the line 0 that no position has: the end of the source.
        assert_eq!(offsets.offset(Position::new(9, 1)), 9);
        assert_eq!(offsets.offset(Position::new(u32::MAX, u32::MAX)), 9);
        assert_eq!(offsets.offset(Position::new(0, 1)), 9);
        // The column 0, that no position has either: the start of its line.
        assert_eq!(offsets.offset(Position::new(2, 0)), 6);
    }
}
