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
        match self {
            Self::Up => if gs.run_mode == RunMode::Start {
                gs.commands = gs.startmenu.manage(VirtualKeyCode::Up);
            } 
            else if gs.run_mode == RunMode::Intro {

            }
            else if gs.run_mode == RunMode::Running {
                gs.commands = gs.player.map_move(0, 1);
            },
            
            Self::Down => if gs.run_mode == RunMode::Start {
                gs.commands = gs.startmenu.manage(VirtualKeyCode::Down);
            }
            
            Self::Right => gs.commands = gs.startmenu.manage(VirtualKeyCode::Left),
            Self::Right => gs.commands = gs.startmenu.manage(VirtualKeyCode::Right),
            _ => todo!(),
        }
    }
}

