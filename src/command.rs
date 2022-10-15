use bracket_lib as bracket;
use bracket::prelude::*;
use crate::State;
use crate::RunMode;
use crate::Menu;


pub trait Command {
    fn execute(&self, gs: &mut State);
}

impl Command for VirtualKeyCode {
    fn execute(&self, gs: &mut State) {
        if gs.run_mode == RunMode::Start { 
            match self {
                Self::Up => gs.commands = gs.startmenu.manage(VirtualKeyCode::Up),
                Self::Down => gs.commands = gs.startmenu.manage(VirtualKeyCode::Down),
                Self::Right => gs.commands = gs.startmenu.manage(VirtualKeyCode::Left),
                Self::Right => gs.commands = gs.startmenu.manage(VirtualKeyCode::Right),
                _ => todo!(),
            }
        }
    }
}
