use bracket::prelude::*;
use bracket_lib as bracket;
use std::collections::HashMap;

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
use dialogue::NodeID;

mod mode;
use mode::RunMode;

mod player;
use player::{Player, Skill, Statistics};

mod scene;
use scene::{Scene, SceneID, StageManager};

mod world;
use world::Calendar;

// Gamestate struct, contains all data to update for game
pub struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    menu: Menu,
    dialogue: dialogue::Dialogue,
    sm: StageManager,
    startart: Art,
    log: Vec<String>,
    redraw: bool,
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
}

// Plan on reading the command stream from input
// Mob actions, updating quests and scenes
fn update(_gs: &mut State) {
    // Game Log - Unused for now
    // let mut i = 41;
    // for entry in &gs.log {
    //     ctx.print_color(64, i, RGB::named(WHITE), RGB::named(BLACK), entry);
    //     i += 1;
    //     if i == 62 {
    //         i = 41;
    //     }
    // }
}

// Updates the visuals of the map, menus, UI, and player icon
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
            "Arrow Keys to Move. ENTER to use Menu.",
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
            "Arrow Keys to Move Menu Selection. ENTER to return to Map Travel.",
        );
        gs.menu.draw(ctx);
    } else if gs.run_mode == RunMode::Storytelling {
        ctx.cls();
        let scene = &gs.sm.scenes[gs.sm.onstage.index];

        if scene.fullscreen == true {
            scene.draw_fullscreen(ctx);
        } else {
            scene.draw_halfscreen(ctx);
            ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
            gs.dialogue.draw(ctx);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(128, 64)
        .unwrap()
        .with_title("Curse Quest")
        .with_automatic_console_resize(false)
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

    let mut sm = StageManager::new(1, vec![], SceneID { index: 0 });

    let title = Art::new("assets/title.txt", String::from("Curse Quest"));

    let game_log = Vec::new();

    let raw_world_map = Map::load("assets/worldmap.txt");
    let map = Map::new(raw_world_map);

    let mut dialogue = dialogue::Dialogue::new();
  
    let mut gs: State = State {
        player,
        map,
        run_mode: RunMode::Start,
        menu: start_menu,
        dialogue: dialogue,
        sm,
        startart: title,
        log: game_log,
        redraw: false,
    };

    let prologue = init::prologue();
    let shir = init::nshir(&mut gs);

    gs.sm.register_scene(prologue);
    gs.sm.register_scene(shir);

    main_loop(context, gs)
}
