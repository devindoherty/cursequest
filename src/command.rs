use bracket_lib as bracket;
use bracket::prelude::*;
use crate::State;
use crate::RunMode;


pub trait Command {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm);
}

impl Command for VirtualKeyCode {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm) -> (){
        match self {
            // UP KEY
            Self::Up => {
                if gs.run_mode == RunMode::Start {
                    gs.commands.push(gs.startmenu.manage(VirtualKeyCode::Up));
                } 
                if gs.run_mode == RunMode::Running {
                    gs.commands.push(gs.player.map_move(0, -1));
                }
                if gs.run_mode == RunMode::Prompting {
                    gs.commands.push(gs.menu.manage(VirtualKeyCode::Up))
                }
            },
            
            // DOWN KEY
            Self::Down => {
                if gs.run_mode == RunMode::Start {
                    gs.commands.push(gs.startmenu.manage(VirtualKeyCode::Down));
                }
                if gs.run_mode == RunMode::Running {
                    gs.commands.push(gs.player.map_move(0, 1));
                }
                if gs.run_mode == RunMode::Prompting {
                    gs.commands.push(gs.menu.manage(VirtualKeyCode::Down))
                }
            },

            // LEFT KEY
            Self::Left => if gs.run_mode == RunMode::Running {
                gs.commands.push(gs.player.map_move(-1, 0));
            },

            // Right KEY
            Self::Right => if gs.run_mode == RunMode::Running {
                gs.commands.push(gs.player.map_move(1, 0));
            },

            // ENTER KEY
            Self::Return => {
                let mut enter = ();
                if gs.run_mode == RunMode::Start {
                    if gs.startmenu.selected == 0 {
                        enter = RunMode::new(gs, RunMode::Intro);
                        return enter;
                    } else {
                        ctx.quit();
                    }
                }
                if gs.run_mode == RunMode::Intro {
                    enter = RunMode::new(gs, RunMode::Running);
                    return enter;
                }
                if gs.run_mode == RunMode::Running {
                    enter = RunMode::new(gs, RunMode::Prompting);
                    return enter;
                }
                if gs.run_mode == RunMode::Prompting {
                    return RunMode::new(gs, RunMode::Running);
                }
                gs.commands.push(enter);
            }

            // OTHER
            _ => {
                if gs.run_mode == RunMode::Intro {
                    gs.run_mode = RunMode::Running;
                }
            }
        }
    }
}




