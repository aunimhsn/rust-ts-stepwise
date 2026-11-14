//! What each built-in function does when a program calls it (see *Built-in functions* in
//! `specs.md`): the one door between the language and the world of the robot.

use super::error::RuntimeErrorKind;
use super::event::{Action, Event};
use super::value::Value;
use crate::builtin::Builtin;
use crate::color::Color;
use crate::world::{Side, World};

/// What a call of a built-in function did. It is one or the other, never both: what gives
/// back no value is the only thing a step shows, which is why a step reports at most one
/// event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Outcome {
    /// An action of the robot, or `print`: what the learner sees of it. It gives back no
    /// value.
    Acted(Event),
    /// A sensor: the boolean it gives back. The learner sees nothing of it.
    Sensed(bool),
}

/// Calls `builtin` with `values`, the ones of the call in order, on `world`.
///
/// # Errors
///
/// [`RuntimeErrorKind::Robot`] when the robot can't do the action,
/// [`RuntimeErrorKind::ValueNotColor`] when a function that takes a color is given another
/// value, and [`RuntimeErrorKind::Internal`] for a wrong number of values, which the checks
/// before running rule out. The world then stays as it was.
pub(super) fn call(
    builtin: Builtin,
    values: &[Value],
    world: &mut World,
) -> Result<Outcome, RuntimeErrorKind> {
    let expected = builtin.parameter_count();
    if expected.is_some_and(|count| count != values.len()) {
        return Err(RuntimeErrorKind::Internal);
    }
    // Every function is listed, so that a new one can't be forgotten, and each arm says
    // whether it acts or senses.
    Ok(match builtin {
        Builtin::Forward => {
            world.forward()?;
            acted(Action::Moved)
        }
        Builtin::TurnLeft => {
            world.turn_left();
            acted(Action::TurnedLeft)
        }
        Builtin::TurnRight => {
            world.turn_right();
            acted(Action::TurnedRight)
        }
        Builtin::StartPainting => {
            world.start_painting(color(builtin, values)?);
            acted(Action::StartedPainting)
        }
        Builtin::StopPainting => {
            world.stop_painting();
            acted(Action::StoppedPainting)
        }
        Builtin::StartErasing => {
            world.start_erasing();
            acted(Action::StartedErasing)
        }
        Builtin::StopErasing => {
            world.stop_erasing();
            acted(Action::StoppedErasing)
        }
        Builtin::PickUp => {
            world.pick_up()?;
            acted(Action::PickedUp)
        }
        Builtin::Drop => {
            world.drop_item()?;
            acted(Action::Dropped)
        }
        Builtin::FrontIsClear => Outcome::Sensed(world.is_clear(Side::Front)),
        Builtin::LeftIsClear => Outcome::Sensed(world.is_clear(Side::Left)),
        Builtin::RightIsClear => Outcome::Sensed(world.is_clear(Side::Right)),
        Builtin::HereIs => Outcome::Sensed(world.paint_here() == Some(color(builtin, values)?)),
        Builtin::FrontIs => Outcome::Sensed(is_painted(world, Side::Front, builtin, values)?),
        Builtin::LeftIs => Outcome::Sensed(is_painted(world, Side::Left, builtin, values)?),
        Builtin::RightIs => Outcome::Sensed(is_painted(world, Side::Right, builtin, values)?),
        Builtin::ItemHere => Outcome::Sensed(world.item_here()),
        Builtin::ItemInFront => Outcome::Sensed(world.item_in_front()),
        Builtin::IsCarrying => Outcome::Sensed(world.is_carrying()),
        Builtin::IsPainting => Outcome::Sensed(world.is_painting()),
        Builtin::IsErasing => Outcome::Sensed(world.is_erasing()),
        Builtin::Print => Outcome::Acted(Event::Printed(printed_line(values))),
    })
}

/// What an action of the robot that went well did.
const fn acted(action: Action) -> Outcome {
    Outcome::Acted(Event::Robot(action))
}

/// `front_is(color)`, `left_is(color)` and `right_is(color)`: whether the cell on `side` has
/// the paint the call gives `function`.
fn is_painted(
    world: &World,
    side: Side,
    function: Builtin,
    values: &[Value],
) -> Result<bool, RuntimeErrorKind> {
    Ok(world.paint_toward(side) == Some(color(function, values)?))
}

/// The color a call gives `function`, its one value.
fn color(function: Builtin, values: &[Value]) -> Result<Color, RuntimeErrorKind> {
    match values {
        [Value::Color(color)] => Ok(*color),
        [other] => {
            let found = other.value_type();
            Err(RuntimeErrorKind::ValueNotColor { function, found })
        }
        _ => Err(RuntimeErrorKind::Internal),
    }
}

/// The line `print` writes for `values`: each as it is written, separated by spaces.
fn printed_line(values: &[Value]) -> String {
    let written: Vec<String> = values.iter().map(Value::to_string).collect();
    written.join(" ")
}
