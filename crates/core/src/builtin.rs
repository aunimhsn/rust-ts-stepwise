//! The built-in functions of the language: what the robot does and senses, and `print`.

use crate::word_enum::word_enum;

word_enum! {
    /// A built-in function (see *Built-in functions* in `specs.md`). The names are reserved:
    /// no variable, function or parameter takes one.
    pub enum Builtin {
        /// `forward()`: moves the robot one cell in front.
        Forward => "forward",
        /// `turn_left()`: turns the robot a quarter turn to its left.
        TurnLeft => "turn_left",
        /// `turn_right()`: turns the robot a quarter turn to its right.
        TurnRight => "turn_right",
        /// `start_painting(color)`: puts the brush down with a color.
        StartPainting => "start_painting",
        /// `stop_painting()`: lifts the brush.
        StopPainting => "stop_painting",
        /// `start_erasing()`: puts the eraser down.
        StartErasing => "start_erasing",
        /// `stop_erasing()`: lifts the eraser.
        StopErasing => "stop_erasing",
        /// `pick_up()`: picks up the item of the cell of the robot.
        PickUp => "pick_up",
        /// `drop()`: drops the item the robot carries.
        Drop => "drop",
        /// `front_is_clear()`: whether the cell in front is a floor cell.
        FrontIsClear => "front_is_clear",
        /// `left_is_clear()`: whether the cell on the left is a floor cell.
        LeftIsClear => "left_is_clear",
        /// `right_is_clear()`: whether the cell on the right is a floor cell.
        RightIsClear => "right_is_clear",
        /// `here_is(color)`: whether the cell of the robot has a paint.
        HereIs => "here_is",
        /// `front_is(color)`: whether the cell in front has a paint.
        FrontIs => "front_is",
        /// `left_is(color)`: whether the cell on the left has a paint.
        LeftIs => "left_is",
        /// `right_is(color)`: whether the cell on the right has a paint.
        RightIs => "right_is",
        /// `item_here()`: whether the cell of the robot has an item.
        ItemHere => "item_here",
        /// `item_in_front()`: whether the cell in front has an item.
        ItemInFront => "item_in_front",
        /// `is_carrying()`: whether the robot carries an item.
        IsCarrying => "is_carrying",
        /// `is_painting()`: whether the brush is down.
        IsPainting => "is_painting",
        /// `is_erasing()`: whether the eraser is down.
        IsErasing => "is_erasing",
        /// `print(value, …)`: writes its values to the console, on one line.
        Print => "print",
    }
}

impl Builtin {
    /// The number of values a call gives the function, or `None` for `print`, which takes
    /// any number of them.
    #[must_use]
    pub const fn parameter_count(self) -> Option<usize> {
        match self {
            Self::Forward
            | Self::TurnLeft
            | Self::TurnRight
            | Self::StopPainting
            | Self::StartErasing
            | Self::StopErasing
            | Self::PickUp
            | Self::Drop
            | Self::FrontIsClear
            | Self::LeftIsClear
            | Self::RightIsClear
            | Self::ItemHere
            | Self::ItemInFront
            | Self::IsCarrying
            | Self::IsPainting
            | Self::IsErasing => Some(0),
            Self::StartPainting | Self::HereIs | Self::FrontIs | Self::LeftIs | Self::RightIs => {
                Some(1)
            }
            Self::Print => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Builtin;

    #[test]
    fn the_number_of_values_of_each_built_in_function() {
        let with_one_value = [
            Builtin::StartPainting,
            Builtin::HereIs,
            Builtin::FrontIs,
            Builtin::LeftIs,
            Builtin::RightIs,
        ];
        for builtin in Builtin::ALL {
            let expected = if builtin == Builtin::Print {
                None
            } else if with_one_value.contains(&builtin) {
                Some(1)
            } else {
                Some(0)
            };
            assert_eq!(builtin.parameter_count(), expected, "{}", builtin.as_str());
        }
    }

    #[test]
    fn the_built_in_functions_of_the_specifications() {
        let words: Vec<_> = Builtin::ALL.into_iter().map(Builtin::as_str).collect();
        assert_eq!(
            words,
            [
                "forward",
                "turn_left",
                "turn_right",
                "start_painting",
                "stop_painting",
                "start_erasing",
                "stop_erasing",
                "pick_up",
                "drop",
                "front_is_clear",
                "left_is_clear",
                "right_is_clear",
                "here_is",
                "front_is",
                "left_is",
                "right_is",
                "item_here",
                "item_in_front",
                "is_carrying",
                "is_painting",
                "is_erasing",
                "print",
            ]
        );
    }
}
