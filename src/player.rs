use bracket_lib as bracket;
use bracket::prelude::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {

    pub fn draw(player: &Player, ctx: &mut BTerm) {
        ctx.set(player.x, player.y, RGB::named(GREEN), RGB::from_f32(0., 0., 0.), to_cp437('@'))
    }

    pub fn map_move(&mut self, xmv: i32, ymv: i32) {
        if self.check_valid_move() {
            self.x += xmv;
            self.y += ymv;
        }
        println!("px: {}, py: {}", self.x, self.y);
    }

    fn check_valid_move(&mut self) -> bool {
        let mut valid: bool = true;
        if self.x > 19  || self. y > 20{
            valid = false;
        }

        valid
    }

}

