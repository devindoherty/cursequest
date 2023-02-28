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
        let menu = &mut gs.menu;
        let dialogue = &mut gs.dialogue;

        // START
        if gs.run_mode == RunMode::Start {
            match self {
                Self::Up | Self::Numpad8 | Self::Down | Self::Numpad2 => menu.manage(*self),
                Self::Return => {
                    if menu.selected == 0 {
                        return gs.run_mode = RunMode::Storytelling;
                    }
                    if menu.selected == 1 {
                        menu.items[1].display_name = "Continue (Not Implemented Yet!)".to_string();
                    }
                    if menu.selected == 2 {
                        ctx.quit();
                    }
                }
                _ => (),
            }
        }

        // STORYTELLING
        if gs.run_mode == RunMode::Storytelling {
            if gs.scene.fullscreen == true {
                match self {
                    _ => {
                        let mm = init::main_menu();
                        *menu = menu.switch(mm);
                        gs.run_mode = RunMode::Prompting;
                    }
                }
            } else if gs.scene.fullscreen == false {
                match self {
                    Self::Up | Self::Numpad8 | Self::Down | Self::Numpad2 | Self::Return => {
                        dialogue.manage(*self)
                    }
                    _ => (),
                }
            }
        }

        // PROMPTING Main In-Game Menu
        if gs.run_mode == RunMode::Prompting {
            match self {
                Self::Up | Self::Numpad8 | Self::Down | Self::Numpad2 | Self::Return => {
                    menu.manage(*self)
                }
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
