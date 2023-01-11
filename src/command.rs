use bracket_lib as bracket;
use bracket::prelude::*;
use crate::init;
use crate::State;
use crate::RunMode;


pub trait Command {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm);
}

impl Command for VirtualKeyCode {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm) -> (){
        match self {
            // UP KEY
            Self::Up | Self::Numpad8 => {
                if gs.run_mode == RunMode::Start {
                    gs.menu.manage(*self);
                } 
                if gs.run_mode == RunMode::Prologue {
                    RunMode::new(gs, RunMode::Travelling);
                }
                if gs.run_mode == RunMode::Travelling {
                    gs.player.map_move(0, -1);
                }
                if gs.run_mode == RunMode::Prompting {
                    gs.menu.manage(*self)
                }
                if gs.run_mode == RunMode::Storytelling {
                    gs.menu.manage(*self)
                }
            },
            
            // DOWN KEY
            Self::Down | Self::Numpad2 => {
                if gs.run_mode == RunMode::Start {
                    gs.menu.manage(VirtualKeyCode::Down);
                }
                if gs.run_mode == RunMode::Travelling {
                    gs.player.map_move(0, 1);
                }
                if gs.run_mode == RunMode::Prompting {
                    gs.menu.manage(VirtualKeyCode::Down)
                }
                if gs.run_mode == RunMode::Storytelling {
                    gs.menu.manage(*self)
                }
            },

            // LEFT KEY
            Self::Left | Self::Numpad4 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(-1, 0);
            },

            // RIGHT KEY
            Self::Right | Self::Numpad6 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(1, 0);
            },

            // DIAGONAL KEYS
            Self::Numpad1 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(-1, 1);
            }
            
            Self::Numpad3 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(1, 1);
            }

            Self::Numpad7 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(-1, -1);
            },

            Self::Numpad9 => if gs.run_mode == RunMode::Travelling {
                gs.player.map_move(1, -1);
            },

            // ENTER KEY
            Self::Return => {
                if gs.run_mode == RunMode::Start {
                    if gs.menu.selected == 0 {
                        return RunMode::new(gs, RunMode::Prologue);
                    } 
                    else if gs.menu.selected == 1 {
                        gs.menu.items[1].display_name = "Continue (Sorry! Not Implemented Yet)".to_string();
                    }
                    else if gs.menu.selected == 2 {
                        return RunMode::new(gs, RunMode::NMenu);
                    }
                    else {
                        ctx.quit();
                    }
                }
                if gs.run_mode == RunMode::Prologue {
                    gs.scene = init::shir();
                    let scene_menu = init::shir().encounter.unwrap().items;
                    gs.menu = gs.menu.switch(scene_menu);
                    return RunMode::new(gs, RunMode::Storytelling);
                }
                if gs.run_mode == RunMode::Travelling {
                    gs.menu = gs.menu.switch(init::travel_menu());
                    return RunMode::new(gs, RunMode::Prompting);
                }
                if gs.run_mode == RunMode::Prompting {
                    return RunMode::new(gs, RunMode::Travelling);
                }
                if gs.run_mode == RunMode::Storytelling {
                    gs.menu.manage(*self)
                }
            }

            // Self::J => {
            //     if gs.run_mode == RunMode::Travelling {
            //         gs.scene = init::shir();
            //         let scene_menu = init::shir().encounter.unwrap().menu;
            //         gs.menu = gs.menu.switch(scene_menu);
            //         // println!("{}", gs.menu.items[0].display_name);
            //         return RunMode::new(gs, RunMode::Storytelling);
            //     }
            // }

            // B KEY
            Self::B => {
                if gs.run_mode == RunMode::Prologue {
                    RunMode::new(gs, RunMode::Start);
                }
                if gs.run_mode == RunMode::Travelling {
                    gs.menu = gs.menu.restore();
                    RunMode::new(gs, RunMode::Prologue);
                    // gs.menu.pop_menu();
                }
                if gs.run_mode == RunMode::Prompting || gs.run_mode == RunMode::Storytelling{
                    // RunMode::new(gs, RunMode::Travelling);
                    gs.menu = gs.menu.restore();
                }
            }

            // OTHER
            _ => {
                if gs.run_mode == RunMode::Prologue {
                    gs.run_mode = RunMode::Travelling;
                }
            }
        }
    }
}