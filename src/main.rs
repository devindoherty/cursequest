use std::fs::File;
use std::io::Read;
use std::path::Path;

use bracket_lib as bracket;
use bracket::prelude::*;

mod encounter;
use encounter::Encounter;
use encounter::Kind;

mod map;
use map::Map;

mod menu;

mod player;
use player::Player;

struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
    // encounters: Vec<Encounter>,
    current_encounter: Encounter,
}


#[derive(PartialEq)]
enum RunMode {
    Intro,
    Chargen, 
    Running, 
    Waiting, 
    Menu, 
    Cinematic
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        input(self, ctx);
        update(self);
        render(self, ctx);
    }
}

fn input(gs: &mut State, ctx: &mut BTerm) {
    if gs.run_mode == RunMode::Intro {
        match ctx.key {
            None => {}
            Some(key) => match key {
              VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quit(),
              _ => gs.run_mode = RunMode::Running,
            }
        }
    }  
    else {
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Left => if gs.player.x > 0 {gs.player.x -= 1},
                VirtualKeyCode::Right => if gs.player.x < 19 {gs.player.x += 1},
                VirtualKeyCode::Up => if gs.player.y > 0 { gs.player.y -= 1},
                VirtualKeyCode::Down => if gs.player.y < 19 {gs.player.y += 1},
                VirtualKeyCode::Escape => ctx.quit(),
                _ => {}
            }
        }
    }
}

fn update(gs: &mut State) {
    println!("Player Position (x: {}, y: {}", gs.player.x, gs.player.y);
}

fn render(gs: &mut State, ctx: &mut BTerm) {
    if gs.run_mode == RunMode::Intro {
        let mut draw_batch = DrawBatch::new();
        // println!("{:#?}", ctx.get_char_size());
        ctx.cls();
        draw_batch.cls();
        let mut block = TextBlock::new(1, 0, 126, 21);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered("Curse Quest")
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln().ln()
            .line_wrap(
                "Hail Kryll of Klathia, true heir to the Elder Throne!")
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
    }
    else { // gs.run_mode == RunMode::Running {
        ctx.cls();
        Map::draw(&gs.map.atlas, ctx);
        Player::draw(&gs.player, ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
    }
}

fn load_ascii_art(art: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(art).expect("Error opening file!");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Error reading file!");
    Ok(data)
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

    let gs: State = State {
        player: player,
        map: world_map,
        run_mode: RunMode::Intro,
        // encounters: act1,
        current_encounter: introduction,
    };

    let king = load_ascii_art(Path::new("assets/king.txt"));
    // println!("{:?}", king);
    main_loop(context, gs)
}
