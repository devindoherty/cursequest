use bracket_lib as bracket;
use bracket::prelude::*;

use crate::State;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub health: i32,
    pub magic: i32,
    pub stats: Statistics,
}

pub struct Journal {
    
}

pub struct Statistics {
    pub intellect: i32,
    pub might: i32,
    pub soul: i32,
}


impl Player {

    pub fn draw(player: &Player, ctx: &mut BTerm) {
        ctx.set(player.x, player.y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('@'))
    }

    pub fn map_move(&mut self, xmv: i32, ymv: i32) {
        if self.check_valid_move(xmv, ymv) {
            self.x += xmv;
            self.y += ymv;
        }
        println!("px: {}, py: {}", self.x, self.y);
    }

    fn check_valid_move(&mut self, xmv: i32, ymv: i32) -> bool {
        let mut valid: bool = true;
        if self.x + xmv > 127  || self.x + xmv < 0 || self. y + ymv > 39 || self. y + ymv < 0 {
            valid = false;
        }
        valid
    }

    fn check_location(&mut self, gs: &mut State, ctx: &mut BTerm, xmv: i32, ymv: i32) {
        for tile in &gs.map.atlas {
            if gs.player.x == tile.x && gs.player.y == tile.y {
                ctx.print_color(1, 42, RGB::named(WHITE), RGB::named(BLACK), &tile.desc);
            }
        }
    }

    fn _take_inventory(&mut self) {

    }

    fn _status(&mut self) {
        
    }
}

