use bracket_lib as bracket;
use bracket::prelude::*;

mod map;
use map::Map;

mod menu;

mod player;
use player::Player;

struct State {
    player: Player,
    map: Map,
    run_mode: RunMode,
}

enum RunMode {Intro, Running, Waiting, Menu, Cinematic}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let king = r"            .
                            / \
                           _\ /_
                 .     .  (,'v`.)  .     .
                 \)   ( )  ,' `.  ( )   (/
                  \`. / `-'     `-' \ ,'/
                   : '    _______    ' :
                   |  _,-'  ,-.  `-._  |
                   |,' ( )__`-'__( ) `.|
                   (|,-,'-._   _.-`.-.|)
                   /  /<( o)> <( o)>\  \
                   :  :     | |     :  :
                   |  |     ; :     |  |
                   |  |    (.-.)    |  |
                   |  |  ,' ___ `.  |  |
                   ;  |)/ ,'---'. \(|  :
               _,-/   |/\(       )/\|   \-._
         _..--'.-(    |   `-'''-'   |    )-.`--.._
                  `.  ;`._________,':  ,'
                 ,' `/               \'`.
                      `------.------'             ";


        let mut draw_batch = DrawBatch::new();
        // println!("{:#?}", ctx.get_char_size());
        ctx.cls();
        draw_batch.cls();
        
        let mut block = TextBlock::new(1, 41, 126, 21);
        let mut buf = TextBuilder::empty();
        
        buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered("Curse Quest")
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln().ln()
            .line_wrap(
                "Hail Kryll of Klathia, true heir to the Klathian Throne!")
            .ln().ln()
            .line_wrap(
                "A decade ago, the Uncrowned King usurped the title from you and banished you to a life of exile. In your wandering you have come across a legendary magical sword. But instead of granting you the power you need to slay the Uncrowned King and reclaim your throne, the sword has Cursed you. You must find a way to lift the Curse or you will fall under the evil sword's malicious control...")
            .ln().ln()
            .line_wrap(
                "Press any key to continue...")
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    
        input(self, ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
        // ctx.print(1, 1, king);
        render(self, ctx);
    }
}


fn input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => gs.player.x -= 1,
            VirtualKeyCode::Right => gs.player.x += 1,
            VirtualKeyCode::Up => gs.player.y -= 1,
            VirtualKeyCode::Down => gs.player.y += 1,
            VirtualKeyCode::Escape => ctx.quit(),
            _ => {}
        }
    }
}


fn update(gs: &mut State, ctx: BTerm) {
    
}


fn render(gs: &mut State, ctx: &mut BTerm) {
    Map::draw(&gs.map.atlas, ctx);
    Player::draw(&gs.player, ctx);
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

    let gs: State = State {
        player: player,
        map: world_map,
        run_mode: RunMode::Intro,
    };

    main_loop(context, gs)
}
