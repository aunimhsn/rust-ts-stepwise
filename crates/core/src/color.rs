//! The colors of the paints of the world.

use crate::word_enum::word_enum;

word_enum! {
    /// A color of paint, written by its name in a program (`start_painting(red)`). The names
    /// are reserved: no variable, function or parameter takes one.
    pub enum Color {
        /// `red`
        Red => "red",
        /// `yellow`
        Yellow => "yellow",
        /// `blue`
        Blue => "blue",
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn the_colors_of_the_specifications() {
        let words: Vec<_> = Color::ALL.into_iter().map(Color::as_str).collect();
        assert_eq!(words, ["red", "yellow", "blue"]);
    }
}
