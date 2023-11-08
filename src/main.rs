#![allow(warnings)] // turning off comp warnings for now REMEMBER TO REMOVE

use bracket::prelude::*;
use bracket_lib as bracket;
// use winit::window::Icon; Trying to set an icon TODO

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
use dialogue::{Dialogue, NodeID, };

mod mode;
use mode::{RunMode};

mod player;
use player::{Player, Skill, Statistics};

mod scene;
use scene::{SceneID, StageManager};

mod world;


// Gamestate struct, contains all data to update for game
pub struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    menu: Menu,
    sm: StageManager,
    startart: Art,
    log: Vec<String>,
    redraw: bool,
    update: bool,
}

impl State {

}

// Bracket required implementation for the Gamestate
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        input(self, ctx);
        update(self);
        render(self, ctx);
    }
}

// Reads input from the terminal construct and handles input via command.execute.
fn input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quit(),
            _ => key.execute(gs, ctx),
        },
    }

    println!("Mouse: {:?}", ctx.mouse_pos);


}

// Plan on reading the command stream from input
// Mob actions, updating quests and scenes
fn update(gs: &mut State) {
    if gs.redraw == false {
        println!("Update: Redraw Not Needed");
    }
    if gs.run_mode == RunMode::Storytelling && gs.sm.onstage.index > 0 {
        let updated_text = gs.sm.scenes[gs.sm.onstage.index]
                            .dialogue.as_ref().unwrap()
                            .items[
                                gs.sm.scenes[gs.sm.onstage.index]
                                .dialogue.as_ref().unwrap()
                                .current.index
                            ]
                            .response.clone();
        let mut scene = &mut gs.sm.scenes[gs.sm.onstage.index];
        if updated_text == "END"{
            gs.menu = gs.menu.switch(init::main_menu());
            gs.run_mode = RunMode::Travelling;
            println!("Menu: {:?}", gs.menu);
        }
        if updated_text == "START COMBAT" {
            // gs.run_mode = RunMode::Combat::PlayerTurn;
        }
        scene.update_text(updated_text);
    }

}

// Updates the visuals of the map, menus, UI, and player icon
fn render(gs: &mut State, ctx: &mut BTerm) {
    if gs.redraw == false {
        return (); // Do Nothing
    }
    
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
        // ctx.cls();
        let scene = &mut gs.sm.scenes[gs.sm.onstage.index];
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
    };

    let start_menu = init::start_menu();

    let sm = StageManager::new(1, vec![], SceneID { index: 0 }, vec![]);

    let title = Art::new("assets/title.txt", String::from("Curse Quest"));

    let game_log = Vec::new();

    let raw_world_map = Map::load("assets/worldmap.txt");
    let map = Map::new(raw_world_map);
  
    let mut gs: State = State {
        player,
        map,
        run_mode: RunMode::Start,
        menu: start_menu,
        sm,
        startart: title,
        log: game_log,
        redraw: true,
        update: false,
    };

    let prologue = init::prologue();
    let shir = init::shir();

    gs.sm.register_scene(prologue);
    gs.sm.register_scene(shir);

    main_loop(context, gs)
}
