use crate::init;
use crate::NodeID;
use crate::RunMode;
use crate::State;
use bracket::prelude::*;
use bracket_lib as bracket;

pub trait Command {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm);
}

impl Command for VirtualKeyCode {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm) -> () {
        let menu_item = &gs.menu.items[gs.menu.selected];

        // START
        if gs.run_mode == RunMode::Start {
            match self {
                Self::Up      | 
                Self::Numpad8 | 
                Self::Down    | 
                Self::Numpad2 => gs.menu.manage(*self),
                Self::Return  => {
                    if gs.menu.selected == 0 {
                        return gs.run_mode = RunMode::Storytelling;
                    }
                    if gs.menu.selected == 1 {
                        gs.menu.items[1].display_name = "Continue (Not Implemented Yet!)".to_string();
                    }
                    if gs.menu.selected == 2 {
                        ctx.quit();
                    }
                }
                _ => (),
            }
        }

        // STORYTELLING
        if gs.run_mode == RunMode::Storytelling {
            let scene = &gs.sm.scenes[gs.sm.onstage.index];
            if scene.fullscreen == true {
                match self {
                    _ => {
                        // let mm = init::main_menu();
                        // gs.menu = gs.menu.switch(mm);
                        // gs.run_mode = RunMode::Prompting;
                        gs.sm.next_scene();
                    }
                }
            } else { // Fullscreen = false
                match self {
                    Self::Up      | 
                    Self::Numpad8 | 
                    Self::Down    | 
                    Self::Numpad2 | 
                    Self::Return  => {
                        gs.dialogue.manage(*self)
                    },
                    Self::D => {
                        for item in &gs.dialogue.items {
                            println!("Dialogue: {:?}", item);
                        }
                    }
                    _ => (),
                }
            }
        }

        // PROMPTING Main In-Game Menu
        if gs.run_mode == RunMode::Prompting {
            match self {
                Self::Up      | 
                Self::Numpad8 | 
                Self::Down    | 
                Self::Numpad2 | 
                Self::Return  => {
                    gs.menu.manage(*self)
                },
                _ => (),
            }
        }

        // TRAVELLING ON WORLD MAP
        if gs.run_mode == RunMode::Travelling {
            match self {
                // ARROW AND NUMPAD DIRECTIONAL KEYS
                Self::Up | Self::Numpad8 => {
                    gs.player.map_move(0, -1);
                }
                Self::Down | Self::Numpad2 => {
                    gs.player.map_move(0, 1);
                }
                Self::Left | Self::Numpad4 => {
                    gs.player.map_move(-1, 0);
                }
                Self::Right | Self::Numpad6 => {
                    gs.player.map_move(1, 0);
                }
                // DIAGONAL KEYS
                Self::Numpad1 => {
                    gs.player.map_move(-1, 1);
                }
                Self::Numpad3 => {
                    gs.player.map_move(1, 1);
                }
                Self::Numpad7 => {
                    gs.player.map_move(-1, -1);
                }
                Self::Numpad9 => {
                    gs.player.map_move(1, -1);
                }
                // Enter
                Self::Return => println!("Test"),
                _ => (),
            }
        }
    }
}
