use bracket_lib as bracket;
use bracket::prelude::*;

mod art;
use art::*;

mod command;
use command::Command;

mod init;

mod encounter;
use encounter::Encounter;

mod map;
use map::Map;

mod menu;
use menu::{Menu, MenuItem};

mod mode;
use mode::RunMode;

mod player;
use player::Player;

// Gamestate struct, contains all data to update for game
pub struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    // encounters: Vec<Encounter>,
    // current_encounter: Encounter,
    menu: Menu,
    startmenu: Menu,
    travelmenu: Menu,
    art: Vec<String>,
    startart: Vec<String>,
    log: Vec<String>,
    commands: Vec<()>,
}

// Bracket required implementation for the Gamestate
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        input(self, ctx);
        update(self);
        render(self, ctx);
    }
}

// Reads input from the terminal construct and handles the input for updating
fn input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quit(),
            _ => key.execute(gs, ctx),
        }
    }
}

// Plan on reading the command stream from input
fn update(gs: &mut State) {

    if gs.commands.len() >= 1 {
        for command in &gs.commands {
            command;
        }
    gs.commands.pop();
    }

    // gs.menu.alter(&gs.player, &gs.map)
    // for tile in &gs.map.atlas {
    //     if tile.x == gs.player.x && tile.y == gs.player.y {
    //         let desc = tile.desc.clone();
    //         gs.log.push(desc);
    //     }
    // }
}

// Updates the visuals of the map, menus, UI, and player icon
fn render(gs: &mut State, ctx: &mut BTerm) {
    let mut draw_batch = DrawBatch::new();

    if gs.run_mode == RunMode::Start {
        ctx.cls();
        draw_batch.cls();
        let mut y = 8;
        for line in &gs.startart {
            ctx.print_color(
            16, y, 
                RGB::named(WHITE), RGB::named(BLACK), 
                line.to_string()
            );
            y += 1;
        }
        ctx.print_color(1, 41, RGB::named(WHITE), RGB::named(BLACK), "Choose Thy Fate");
        gs.startmenu.draw(ctx);
    }
    else if gs.run_mode == RunMode::Intro {
        // println!("{:#?}", ctx.get_char_size());
        ctx.cls();
        draw_batch.cls();
        let mut block = TextBlock::new(1, 0, 126, 21);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered("Prologue")
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln().ln()
            .line_wrap(
                "Hail Kryll of Klathia, true heir to the Throne!")
            .ln().ln()
            .line_wrap(
                "A decade ago, the Uncrowned King usurped the title from you and banished you to a life of exile. In your wandering you have come across a legendary magical sword. But instead of granting you the power you need to slay the Uncrowned King and reclaim your throne, the sword has Cursed you. You must find a way to lift the Curse or you will fall under the evil sword's malicious control...")
            .ln().ln()
            .line_wrap(
                "ESC or Q to quit. Press any other key to continue...")
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
        let mut y = 25;
        for line in &gs.art {
            ctx.print_color(
                30, y, 
                RGB::named(WHITE), RGB::named(BLACK), 
                line.to_string()
            );
            y += 1;
        }
    }
    else if gs.run_mode == RunMode::Running {
        ctx.cls();
        Map::draw(&gs.map.atlas, ctx);
        Player::draw(&gs.player, ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print_color(1, 41, RGB::named(WHITE), RGB::named(BLACK), "Arrow Keys to Move. ENTER to use Menu.");
        // gs.menu.draw(ctx);
        for tile in &gs.map.atlas {
            if gs.player.x == tile.x && gs.player.y == tile.y {
                ctx.print_color(1, 42, RGB::named(WHITE), RGB::named(BLACK), &tile.desc);
            }
        }

        let mut i = 41;
        for entry in &gs.log {
            ctx.print_color(64, i, RGB::named(WHITE), RGB::named(BLACK), entry);
            i += 1;
            if i == 62 {
                i = 41;
            }
        }
        gs.travelmenu.draw(ctx);
    }
    else if gs.run_mode == RunMode::Prompting {
        ctx.cls();
        Map::draw(&gs.map.atlas, ctx);
        Player::draw(&gs.player, ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
        ctx.print_color(1, 41, RGB::named(WHITE), RGB::named(BLACK), "Arrow Keys to Move Menu Selection. ENTER to return to Map Travel.");
        gs.menu.draw(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(128, 64)
    .unwrap()
    .with_title("Curse Quest")
    .with_automatic_console_resize(true)
    .build()?;
    
    let player: Player = Player {
        x: 12,
        y: 38,
        health: 100,
        magic: 100,
    };

    let (start_menu, main_menu, travel_menu) = init::init_menus();

    let king = load_ascii_art("assets/king.txt");
    let sword = load_ascii_art("assets/sword.txt");

    let com = Vec::new(); 

    let game_log = Vec::new();

    let raw_world_map = Map::load("assets/worldmap.txt");
    let world_map = Map::new(raw_world_map);

    let gs: State = State {
        player: player,
        map: world_map,
        run_mode: RunMode::Start,
        // encounters: act1,
        // current_encounter: introduction,
        menu: main_menu,
        startmenu: start_menu,
        travelmenu: travel_menu,
        art: king,
        startart: sword,
        log: game_log,
        commands: com,
    };

    main_loop(context, gs)
}
