use bracket_lib as bracket;
use bracket::prelude::*;
use crate::init;
use crate::State;
use crate::RunMode;
use crate::NodeID;


pub trait Command {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm);
}

impl Command for VirtualKeyCode {
    fn execute(&self, gs: &mut State, ctx: &mut BTerm) -> (){
        // START
        if gs.run_mode == RunMode::Start {
            match self {
                Self::Up | Self::Numpad8 | 
                Self::Down | Self::Numpad2 | 
                Self::Return => gs.menu.manage(*self, gs),
                _ => (),
            }
        }
        // PROLOGUE
        if gs.run_mode == RunMode::Prologue {
            match self {
                _ => gs.run_mode.new(RunMode::Storytelling),
            }
        }

        // STORYTELLING
        if gs.run_mode == RunMode::Storytelling {
            match self {
                Self::Up | Self::Numpad8 | 
                Self::Down | Self::Numpad2 | 
                Self::Return => gs.dialogue.manage(*self, gs),
                _ => (),
            }
        }

        // PROMPTING Main In-Game Menu
        if gs.run_mode == RunMode::Prompting {
            match self {
                Self::Up | Self::Numpad8 |
                Self::Down | Self::Numpad2 |
                Self::Return => gs.menu.manage(*self, gs),
                _ => (),
            }
        }

        // TRAVELLING ON WORLD MAP
        if gs.run_mode == RunMode::Travelling {
            match self {
                // ARROW AND NUMPAD DIRECTIONAL KEYS
                Self::Up | Self::Numpad8 => {
                    gs.player.map_move(0, -1);
                },
                Self::Down | Self::Numpad2 => {
                    gs.player.map_move(0, 1);
                },
                Self::Left | Self::Numpad4 => {
                    gs.player.map_move(-1, 0);
                },
                Self::Right | Self::Numpad6 => {
                    gs.player.map_move(1, 0);
                },
                // DIAGONAL KEYS
                Self::Numpad1 => {
                    gs.player.map_move(-1, 1);
                },
                Self::Numpad3 => {
                    gs.player.map_move(1, 1);
                },
                Self::Numpad7 => {
                    gs.player.map_move(-1, -1);
                },
                Self::Numpad9 => {
                    gs.player.map_move(1, -1);
                },
                Self::Return => todo!(),
                _ => (),
            }
        }
    }
}

