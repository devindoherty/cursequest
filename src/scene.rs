use crate::Art;
// use crate::State;
use crate::Encounter;

use bracket_lib as bracket;
use bracket::prelude::*;

pub struct StageManager {
    scenes: Vec<Scene>,
}

pub struct Scene {
    pub title: String,
    pub main: String,
    pub art: Art,
    pub encounter: Option<Encounter>,
}

impl Scene {
    pub fn new(title: String, main: String, art: Art, encounter: Option<Encounter>) -> Self {
        Scene { title, main, art, encounter}
    }

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
            .ln().ln()
            .line_wrap(&self.main)
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
        let mut y = 25;
        for line in &self.art.ascii {
            ctx.print_color(
                30, y, 
                RGB::named(WHITE), RGB::named(BLACK), 
                line.to_string()
            );
            y += 1;
        }
    }

    // Work in progress
    // pub fn draw_halfscreen(&self, _ctx: &mut BTerm) {

    // }

}