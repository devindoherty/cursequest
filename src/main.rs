#![allow(warnings)] // turning off comp warnings for now REMEMBER TO REMOVE
// #![windows_subsystem = "windows"]

use bracket::prelude::*;
use bracket_lib as bracket;
// use winit::window::Icon; TODO: Trying to set an icon 

mod art;
use art::*;

mod command;
use command::Command;

mod init;

mod map;
use map::Map;

mod menu;
use menu::{Menu, MenuItem};

mod dialogue;
use dialogue::{Dialogue, NodeID, Link};

mod flag;
use flag::*;

mod mode;
use mode::{RunMode};

mod player;
use player::{Player, Skill, Statistics};

mod scene;
use scene::{Scene, SceneID, StageManager};

// mod world;

// Gamestate struct, contains all data to update for game
pub struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    menu: Menu,
    sm: StageManager,
    startart: Art,
    log: Vec<String>,
    flags: Flags,
}

impl State {}

// Bracket required implementation for the Gamestate
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if input(self, ctx) == true || self.run_mode == RunMode::Start {
            update(self);
            render(self, ctx);
        }
    }
}

// Reads input from the terminal construct and handles input via command.execute.
fn input(gs: &mut State, ctx: &mut BTerm) -> bool {
    match ctx.key {
        None => {return false},
        Some(key) => match key {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quit(),
            _ => key.execute(gs, ctx),
        },
    }
    true
}

// Plan on reading the command stream from input
// Mob actions, updating quests and scenes
fn update(gs: &mut State) {
    if gs.run_mode == RunMode::Start {

    }
    
    
    if gs.run_mode == RunMode::Storytelling && gs.sm.onstage.index > 0 {
        Scene::update_text(gs);
        Dialogue::update_links(gs);

        let scene_idx = gs.sm.current_scene_id_index();
        let scene = &mut gs.sm.scenes[scene_idx];
        let mut dialogue = scene.dialogue.as_mut().unwrap();
        let dialogue_flags = &mut dialogue.items[dialogue.current.index].flag_names;

        if dialogue_flags.is_some() {
            for flag in &mut gs.flags.flags {
                if flag.name.as_str() == dialogue_flags.as_mut().unwrap() {
                    if flag.flagged == false {
                        flag.flagged = true;
                        println!("Flagged: {:?}", flag);
                    }
                }
            }
        }
        
        // if dialogue.items[dialogue.current.index].link.is_some() {
        //     println!("Link Detected.");
        //     match dialogue.items[dialogue.current.index].link.as_ref().unwrap() {
        //         Link::Remove => println!("Linktype: Remove"),
        //         Link::RemoveSiblings => {
        //             println!("Linktype: Remove Siblings");
        //             // Dialogue::remove_siblings(gs);
        //         }
        //         _ => todo!(),
        //     };
        // }
    }
}

// Renders the visuals of the map, menus, UI, and player icon
fn render(gs: &mut State, ctx: &mut BTerm) {
    if gs.run_mode == RunMode::Start {
        gs.startart.draw(ctx, 16, 8);
        ctx.print_color(
            1,
            41,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Choose Thy Fate",
        );
        gs.menu.draw(ctx);
    } else if gs.run_mode == RunMode::Travelling {
        ctx.cls();
        Map::draw(&gs.map.atlas, ctx);
        gs.player.draw(ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print_color(
            1,
            41,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Directional Keys to move. Enter to use the travel menu.",
        );
        gs.menu.draw(ctx);
    } else if gs.run_mode == RunMode::Prompting {
        ctx.cls();
        // gs.scene.draw_halfscreen(ctx);
        Map::draw(&gs.map.atlas, ctx);
        gs.player.draw(ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print_color(
            1,
            41,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Select Travel to return to Map.",
        );
        gs.menu.draw(ctx);
    } else if gs.run_mode == RunMode::Storytelling {
        let current_scene = gs.sm.current_scene_id_index();
        let scene = &mut gs.sm.scenes[current_scene];
        
        if scene.fullscreen == true {
            scene.draw_fullscreen(ctx);
        } else {
            scene.draw_halfscreen(ctx);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(128, 64)
        .unwrap()
        .with_title("Curse Quest")
        .with_automatic_console_resize(false)
        .with_fullscreen(false)
        .with_fitscreen(true)
        .with_fps_cap(30.0)
        .build()?;

    let player: Player = Player {
        x: 12,
        y: 38,
        health: 100,
        magic: 100,
        stats: Statistics {
            grace: 50,
            might: 50,
            mind: 50,
            soul: 50,
        },
        skills: vec![
            Skill {
                name: String::from("Sword"),
                desc: String::from("Mastery of the Cursed Blade."),
                value: 10,
                abilities: vec![],
            },
            Skill {
                name: String::from("Sorcery"),
                desc: String::from("Knowledge of the secrets of magic."),
                value: 0,
                abilities: vec![],
            }
        ]
    };

    let raw_world_map = Map::load("assets/worldmap.txt");
    let map = Map::new(raw_world_map);
  
    let mut gs: State = State {
        player,
        map,
        run_mode: RunMode::Start,
        menu: init::start_menu(),
        sm: StageManager::new(1, vec![], SceneID { index: 0 }),
        startart: Art::new("assets/title.txt", String::from("Curse Quest")),
        log: Vec::new(),
        flags: init::load_flags(),
    };

    let prologue = init::prologue();
    let shir = init::shir();

    gs.sm.register_scene(prologue);
    gs.sm.register_scene(shir);

    main_loop(context, gs)
}