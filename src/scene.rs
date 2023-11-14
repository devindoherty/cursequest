use bracket_lib as bracket;
use bracket::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Art;
use crate::State;
use crate::Menu;
use crate::Dialogue;

pub struct StageManager {
    act: u8,
    pub scenes: Vec<Scene>,
    pub onstage: SceneID,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub title: String,
    pub text: String,
    pub art: Art,
    pub menu: Option<Menu>,
    pub dialogue: Option<Dialogue>,
    pub fullscreen: bool,
    pub id: SceneID,
}

#[derive(Clone, Debug)]
pub struct SceneID {
    pub index: usize,
}

impl StageManager {
    pub fn new(act: u8, scenes: Vec<Scene>, onstage: SceneID) -> Self {
        StageManager {
            act,
            scenes,
            onstage,
        }
    }

    pub fn register_scene(&mut self, mut scene: Scene) {
        let next_index = self.scenes.len();
        scene.id.index = next_index;
        // println!("StageManager: {} is index {}", scene.title, next_index);
        self.scenes.push(scene);
    }

    pub fn next_scene(&mut self) {
        self.onstage.index += 1;
    }
    
    
    pub fn _stage_scene(&mut self, _gs: &mut State, _onstage_scene: SceneID) {
        ()
    }

    pub fn get_current_scene(&self) -> &Scene {
        &self.scenes[self.onstage.index]
    }

}

impl Scene {
    pub fn new(
        title: String,
        text: String,
        art: Art,
        fullscreen: bool,
        menu: Option<Menu>,
        dialogue: Option<Dialogue>,
        id: SceneID,
    ) -> Self {
        Scene {
            title,
            text,
            art,
            fullscreen,
            menu,
            dialogue,
            id,
        }
    }

    pub fn update_text(&mut self, updated_text: String) {
        self.text = updated_text.to_string();
    }

    // Full Screen cinematic style
    pub fn draw_fullscreen(&self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        ctx.cls();
        draw_batch.cls();
        let mut block = TextBlock::new(1, 0, 126, 21);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .centered(&self.title)
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .ln()
            .line_wrap(&self.text)
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
        self.art.draw(ctx, 30, 25);
    }

    // Half screen within
    pub fn draw_halfscreen(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let mut draw_batch = DrawBatch::new();
        let mut block = TextBlock::new(3, 41, 125, 25);
        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .centered(&self.title)
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .ln()
            .line_wrap(&self.text)
            .reset();
        block.print(&buf).expect("Line too long!");
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
        self.art.draw(ctx, 32, 1);
        self.dialogue.as_mut().expect("Scene Dialogue Missing Error").draw(ctx);
        ctx.draw_hollow_box(0, 40, 127, 22, RGB::named(WHITE), RGB::named(BLACK));
    }
}