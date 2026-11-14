//! What a step of a program did, for the interface to show.

/// What a step did that the learner sees outside the editor. A step does at most one such
/// thing: a function that gives back no value is a statement alone on its line, or stops the
/// program once it has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// `print` wrote this line to the console.
    Printed(String),
    /// The robot did this action, which the map animates.
    Robot(Action),
}

/// An action of the robot, by its name alone: what it changed (the place of the robot, the
/// paint, the items) is read in the world, which the interface draws anyway.
///
/// An action that changes nothing is reported all the same (`stop_painting()` with the brush
/// already up): the interface, which draws the robot, sees that nothing is left to animate.
/// An action that fails is no event: the error that stops the program tells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// `forward()`: it moved one cell in front, painting or erasing with the tool that is
    /// down.
    Moved,
    /// `turn_left()`: it turned a quarter turn to its left.
    TurnedLeft,
    /// `turn_right()`: it turned a quarter turn to its right.
    TurnedRight,
    /// `start_painting(color)`: it put its brush down, or changed its color.
    StartedPainting,
    /// `stop_painting()`: it lifted its brush.
    StoppedPainting,
    /// `start_erasing()`: it put its eraser down.
    StartedErasing,
    /// `stop_erasing()`: it lifted its eraser.
    StoppedErasing,
    /// `pick_up()`: it picked up the item of its cell.
    PickedUp,
    /// `drop()`: it dropped the item it carried on its cell.
    Dropped,
}
