use crate::State;

// Different "modes" for the game
#[derive(Debug, PartialEq)]
pub enum RunMode {
    Start,
    Travelling,
    Prompting,
    Storytelling,
    Updating,
    Combat,
}
