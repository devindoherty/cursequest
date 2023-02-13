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
    pub fn change(&mut self, mode: RunMode){
        self.run_mode = mode;
    }

}