use bracket_lib as bracket;
use bracket::prelude::*;

mod art;
use art::*;

mod command;
use command::Command;

mod encounter;
use encounter::Encounter;
use encounter::Kind;

mod map;
use map::Map;

mod menu;
use menu::{Menu, MenuItem};

mod player;
use player::Player;

// Gamestate struct, contains all data to update for game
struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    // encounters: Vec<Encounter>,
    current_encounter: Encounter,
    menu: Menu,
    startmenu: Menu,
    art: Vec<String>,
    startart: Vec<String>,
    commands: Vec<String>,
}

// Different "modes" for the game
#[derive(PartialEq)]
enum RunMode {
    Start,
    Intro,
    // Chargen, 
    Running, 
    Waiting, 
    Prompting, 
    Scence
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
    let mut player = &mut gs.player;
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quit(),
            VirtualKeyCode::Up => gs.startmenu.manage(ctx, VirtualKeyCode::Up),
            VirtualKeyCode::Down => gs.startmenu.manage(ctx, VirtualKeyCode::Down),
            VirtualKeyCode::Left => VirtualKeyCode::Left.execute(),
            VirtualKeyCode::Return => if gs.startmenu.selected == 0 {
                gs.run_mode = RunMode::Intro;
            } else {
                ctx.quit();
            }
            _ => {}
        }
    }
}


// Plan on reading the command stream from input
fn update(gs: &mut State) {
    // let mut com = &mut gs.commands;
    
    // println!("Player Position (x: {}, y: {})", gs.player.x, gs.player.y);
    // gs.menu_manager.draw();
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
        gs.menu.draw(ctx);
    }
    else if gs.run_mode == RunMode::Prompting {
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
        x: 0,
        y: 0,
    };

    let world_map = Map {
        atlas: vec![
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3,
            2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 3,
            2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
            2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2,
            2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 1, 1, 1,
            1, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1,
            1, 1, 2, 2, 2, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1,
            1, 1, 1, 2, 2, 2, 4, 4, 4, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1,
            1, 1, 1, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1, 1,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 1, 1, 1, 1,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ]
    };

    let introduction = Encounter {
        name: String::from("The Curse Quest"),
        // enemies: None,
        flavor: String::from("Test"),
        kind: Kind::Story,
        art: 2
    };

    // let act1 = vec![introduction];
    let menu_item_one = MenuItem {
        display_name: String::from("Travel"),
        display_char: '1'
    };

    let menu_item_two = MenuItem {
        display_name: String::from("Character"),
        display_char: '2'
    };

    let menu_item_three = MenuItem {
        display_name: String::from("Inventory"),
        display_char: '3'
    };

    let menu_item_four = MenuItem {
        display_name: String::from("Journal"),
        display_char: '4'
    };

    let main_menu = Menu {
        items: vec![menu_item_one, menu_item_two, menu_item_three, menu_item_four],
        selected: 0,
    };

    let start_item_one = MenuItem{
        display_name: String::from("Start"),
        display_char: '0',
    };
    let start_item_two = MenuItem{
        display_name: String::from("Quit"),
        display_char: '1'
    };

    let start_menu = Menu {
        items: vec![start_item_one, start_item_two],
        selected: 0,
    };


    let mut king = load_ascii_art("assets/king.txt");
    let mut sword = load_ascii_art("assets/sword.txt");

    let mut com = Vec::new(); 

    let gs: State = State {
        player: player,
        map: world_map,
        run_mode: RunMode::Start,
        // encounters: act1,
        current_encounter: introduction,
        menu: main_menu,
        startmenu: start_menu,
        art: king,
        startart: sword,
        commands: com,
    };

    
    // println!("{:?}", king);
    main_loop(context, gs)
}
