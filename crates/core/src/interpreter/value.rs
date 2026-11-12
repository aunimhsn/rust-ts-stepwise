//! The values of a program (see *Values* in `specs.md`): what a variable holds and what an
//! expression gives.

use std::fmt;
use std::rc::Rc;

use crate::color::Color;

/// The most characters of two texts joined by `+`: a longer result stops the program.
///
/// Without it, a loop that doubles a text (`line = line + line`) would fill the memory of the
/// page in a few dozen turns, which aborts the WebAssembly module as a panic does. The numbers
/// and the calls have their own limits. A text written in a program may be longer: the size
/// of the program bounds it.
pub const MAX_TEXT_LENGTH: usize = 10_000;

/// A value of a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A whole number.
    Number(i64),
    /// `true` or `false`.
    Boolean(bool),
    /// A text. Shared, so that reading a variable never copies its characters.
    Text(Rc<str>),
    /// A color of paint.
    Color(Color),
}

impl Value {
    /// The type of the value.
    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        match self {
            Self::Number(_) => ValueType::Number,
            Self::Boolean(_) => ValueType::Boolean,
            Self::Text(_) => ValueType::Text,
            Self::Color(_) => ValueType::Color,
        }
    }
}

/// The value as `print` writes it: a text without its quotes, the others as they are written
/// in a program.
impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(formatter, "{number}"),
            Self::Boolean(boolean) => write!(formatter, "{boolean}"),
            Self::Text(text) => formatter.write_str(text),
            Self::Color(color) => formatter.write_str(color.as_str()),
        }
    }
}

/// The type of a value: what an error says of a value an operator or a statement refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueType {
    /// A whole number.
    Number,
    /// A boolean.
    Boolean,
    /// A text.
    Text,
    /// A color.
    Color,
}

#[cfg(test)]
mod tests {
    use super::{Value, ValueType};
    use crate::color::Color;

    #[test]
    fn a_value_is_written_as_print_writes_it() {
        assert_eq!(Value::Number(-7).to_string(), "-7");
        assert_eq!(Value::Number(i64::MIN).to_string(), "-9223372036854775808");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
        assert_eq!(Value::Text("I am stuck".into()).to_string(), "I am stuck");
        assert_eq!(Value::Color(Color::Yellow).to_string(), "yellow");
    }

    #[test]
    fn each_value_has_its_type() {
        assert_eq!(Value::Number(0).value_type(), ValueType::Number);
        assert_eq!(Value::Boolean(false).value_type(), ValueType::Boolean);
        assert_eq!(Value::Text("".into()).value_type(), ValueType::Text);
        assert_eq!(Value::Color(Color::Red).value_type(), ValueType::Color);
    }
}
