use bracket_lib as bracket;
use bracket::prelude::*;
use crate::State;
use crate::RunMode;


pub trait Command {
    fn execute(&self);
    fn travel(&self, gs: State);
}

impl Command for VirtualKeyCode {
    fn execute(&self) {
        println!("Did this work???");
    }
    fn travel(&self, gs: State) {
        if gs.run_mode == RunMode::Start {
            println!("You ain't traveling, you're on the opening menu!");
        }
    }
}