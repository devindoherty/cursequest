use crate::State;

// Different "modes" for the game
#[derive(Debug, PartialEq)]
pub enum RunMode {
    Start,
    Intro,
    // Chargen, 
    Travelling, 
    // Waiting, 
    Prompting, 
    // Scence
}

impl RunMode {
    pub fn new(gs: &mut State, mode: RunMode){
        gs.run_mode = mode;
    }

}