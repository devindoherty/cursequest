use bracket::prelude::*;
use bracket_lib as bracket;

use crate::State;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub health: i32,
    pub magic: i32,
    pub stats: Statistics,
    // pub inv: Inventory,
    // pub journal: Journal,
}

pub struct Journal {}

pub struct Inventory {}

pub struct Statistics {
    pub grace: i32, // Agility and dexterity; change to hit with all attacks
    pub might: i32, // Constitution and strength; damage for all attacks
    pub mind: i32,  // Wisdom and Intellect; magical aptitude and potency
    pub soul: i32,  // Force of personality and ego; interpersonal skills, regen, and divine luck
}

pub struct Skill {
    pub name: String,
    pub desc: String,
    pub value: i32,
    pub abilities: Vec<Ability>,
}

pub struct Ability {
    pub name: String,
    pub desc: String,
}

pub struct Talent {
    pub name: String,
    pub desc: String,
    pub value: i32,
}

impl Player {
    pub fn draw(&self, ctx: &mut BTerm) {
        ctx.set(
            self.x,
            self.y,
            RGB::named(GREEN),
            RGB::named(BLACK),
            to_cp437('@'),
        )
    }

    pub fn map_move(&mut self, xmv: i32, ymv: i32) {
        if self.check_valid_move(xmv, ymv) {
            self.x += xmv;
            self.y += ymv;
            println!("px: {}, py: {}", self.x, self.y);
        } else {
            println!("You cannot travel there.");
        }
    }

    fn check_valid_move(&mut self, xmv: i32, ymv: i32) -> bool {
        let mut valid: bool = true;
        if self.x + xmv > 127 || self.x + xmv < 0 || self.y + ymv > 39 || self.y + ymv < 0 {
            valid = false;
        }
        valid
    }

    fn _check_location(&mut self, gs: &mut State, ctx: &mut BTerm, _xmv: i32, _ymv: i32) {
        for tile in &gs.map.atlas {
            if gs.player.x == tile.x && gs.player.y == tile.y {
                ctx.print_color(1, 42, RGB::named(WHITE), RGB::named(BLACK), &tile.desc);
            }
        }
    }

    fn _take_inventory(&mut self) {}

    fn _status(&mut self) {}
}
