use crate::State;

// Different "modes" for the game
#[derive(Debug, PartialEq)]
pub enum RunMode {
    Start,
    Prologue, // slated for elim
    Travelling, 
    Prompting,
    Storytelling,
    Updating, 
}