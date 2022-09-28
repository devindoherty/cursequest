use bracket_lib as bracket;
use bracket::prelude::*;


pub struct Map {
    pub atlas: Vec<i32>,
}

pub enum _MapTile {
    Blank,
    Plain,
    Grassland,
    Mountain,
    Water,
    Forest,
    Keep,
    City,
    Hall,
    Enclave,
}

// map_legend = {0:" ",
//               1:".",
//               2:",",
//               3:"A",
//               4:"~",
//               5:"t",
//               6:"K",
//               7:"C",
//               8:"H",
//               9:"E",
//               10:"@"}

impl Map {
    
    pub fn draw(map: &[i32], ctx: &mut BTerm){
        let mut y = 0;
        let mut x = 0;
        for tile in map.iter() {
            match tile {
                0 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437(' '));
                }
                1 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
                }
                2 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437(','));
                }
                3 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('A'));
                }
                4 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('~'));
                }
                5 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
                }
                6 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('K'));
                }
                7 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('C'));
                }
                8 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('H'));
                }
                9 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('E'));
                }
                10 => {
                    ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('@'));
                }
                _ => {}
            }
            x += 1;
            if x > 19 {
                x = 0;
                y += 1;
            }
        }
    }
}