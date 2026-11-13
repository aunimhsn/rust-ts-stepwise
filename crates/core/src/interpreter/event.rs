//! What a step of a program did, for the interface to show.

/// What a step did that the learner sees outside the editor. A step does at most one such
/// thing: a function that gives back no value is a statement alone on its line, or stops the
/// program once it has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// `print` wrote this line to the console.
    Printed(String),
}
