use bracket_lib as bracket;
use bracket::prelude::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {

    pub fn draw(player: &Player, ctx: &mut BTerm) {
        ctx.set(player.x, player.y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('@'))
    }
}