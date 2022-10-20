use bracket_lib as bracket;
use bracket::prelude::*;

use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Map {
    pub atlas: Vec<MapTile>,
}

pub struct MapTile {
    icon: char,
    desc: String,
    x: i32,
    y: i32,
    id: i32,
}

pub enum MapType {
    None,
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
    pub fn new(raw_map: Vec<char>) -> Map {
        let mut new_map = Map {
            atlas: Vec::new(),
        };
        let mut x = 0;
        let mut y = 0;
        let mut i = 0;
        for c in raw_map {
            let tile = MapTile {
                icon: c,
                desc: String::from("A Portion of the Land of Klathia"),
                x: x,
                y: y,
                id: i, 
            };
            new_map.atlas.push(tile);
            i += 1;
            x += 1;
            if x > 127 {
                x = 0;
                y += 1;
            }
        }
        new_map
    }

    pub fn load(mapfile: &str) -> Vec<char> {
        let file = File::open(mapfile).expect("Error opening file!");
        let reader = BufReader::new(file);
        let mut raw_map = Vec::new();
        for line in reader.lines() {
            for c in line.expect("lines failed").chars() {
                raw_map.push(c);
            }
        }
        raw_map
    }

    pub fn draw(map: &[MapTile], ctx: &mut BTerm){
        for tile in map.iter() {
            ctx.set(tile.x, tile.y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437(tile.icon));
        }
    }
}