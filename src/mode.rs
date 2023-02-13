use crate::State;

// Different "modes" for the game
#[derive(Debug, PartialEq)]
pub enum RunMode {
    Start,
    Prologue,
    Travelling, 
    Prompting,
    Storytelling,
    Updating, 
}

impl RunMode {
    pub fn new(gs: &mut State, mode: RunMode){
        gs.run_mode = mode;
    }

}