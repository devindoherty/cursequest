use crate::Art;
// use crate::State;
use crate::Menu;
use crate::NodeID;

use bracket::prelude::*;
use bracket_lib as bracket;

pub struct StageManager {
    act: u8,
    scenes: Vec<SceneID>,
    flags: Vec<Flag>,
}

pub struct Flag {
    name: String,
    flagged: bool,
}

pub struct SceneID {
    index: usize,
}

pub struct Scene {
    pub title: String,
    pub main: String,
    pub art: Art,
    pub menu: Option<Menu>,
    pub dialogue: Option<NodeID>,
    pub fullscreen: bool,
    id: SceneID,
}

impl StageManager {}

impl Scene {
    pub fn new(
        title: String,
        main: String,
        art: Art,
        menu: Option<Menu>,
        dialogue: Option<NodeID>,
        fullscreen: bool,
        id: SceneID,
    ) -> Self {
        Scene {
            title,
            main,
            art,
            menu,
            dialogue,
            fullscreen,
            id,
        }
    }

    // Full Screen cinematic style
    pub fn draw_fullscreen(&self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        ctx.cls();
        draw_batch.cls();
        let mut block = TextBlock::new(1, 0, 126, 21);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered(&self.title)
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .ln()
            .line_wrap(&self.main)
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
        self.art.draw(ctx, 30, 25);
    }

    // Half screen within
    pub fn draw_halfscreen(&self, ctx: &mut BTerm) {
        ctx.cls();
        self.art.draw(ctx, 32, 1);
        let mut draw_batch = DrawBatch::new();
        // ctx.cls();
        // draw_batch.cls();
        let mut block = TextBlock::new(32, 41, 96, 21);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered(&self.title)
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .ln()
            .line_wrap(&self.main)
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    }
}
