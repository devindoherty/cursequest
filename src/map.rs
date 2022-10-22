use bracket_lib as bracket;
use bracket::prelude::*;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Map {
    pub atlas: Vec<MapTile>,
}

pub struct MapTile {
    pub icon: char,
    pub desc: String,
    pub x: i32,
    pub y: i32,
    pub biome: Biome,
    id: i32,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Biome {
    City,
    Dungeon,
    Enclave,
    Farm,
    Forest,
    Grassland,
    Hall,
    Keep,
    Mountain,
    Other,
    Plain,
    Road,
    Tundra,
    Village,
    Wasteland,
    Water,
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
                desc: match c {
                    '.' => String::from("Vast plains."),
                    ',' => String::from("Lush grasslands."),
                    'A' | '^' => String::from("Majestic mountains."),
                    '~' => String::from("Bountiful waters."),
                    'f' | 'F' => String::from("Verdant forests."),
                    'K' => String::from("A stout keep."),
                    'C' => String::from("A bustling city."),
                    'M' => String::from("A dwarven mountain hall."),
                    'E' => String::from("An elven forest enclave."),
                    'x' | 'y' => String::from("Blasted wasteland."),
                    'V' | 'H' => String::from("A pronvincial village"),
                    '#' => String::from("Flat farmland."),
                    '=' | '/' | '\\' | '-' | '|' => String::from("A traveller's road."),
                    'D' => String::from("A dangerous dungeon."),
                    '*' => String::from("Windswept tundra."),
                    _ => String::from("The Lands of Klathia."),
                }, 
                x: x,
                y: y,
                id: i,
                biome: match c {
                    '.' => Biome::Plain,
                    ',' => Biome::Grassland,
                    'A' | '^' => Biome::Mountain,
                    '~' => Biome::Water,
                    'f' | 'F' => Biome::Forest,
                    'K' => Biome::Keep,
                    'C' => Biome::City,
                    'M' => Biome::Hall,
                    'E' => Biome::Enclave,
                    'x' | 'y' => Biome::Wasteland,
                    'V' | 'H' => Biome::Village,
                    '#' => Biome::Farm,
                    '=' | '/' | '\\' | '-' | '|' => Biome::Road,
                    'D' => Biome::Dungeon,
                    '*' => Biome::Tundra,
                    _ => Biome::Other,
                } 
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
        let colormap = HashMap::from([
            (Biome::City, RGB::named(FIREBRICK1)),
            (Biome::Dungeon, RGB::named(PURPLE)),
            (Biome::Enclave, RGB::named(GREEN)),
            (Biome::Farm, RGB::named(WHEAT3)),
            (Biome::Forest, RGB::named(FORESTGREEN)),
            (Biome::Grassland, RGB::named(LAWNGREEN)),
            (Biome::Hall, RGB::named(STEELBLUE)),
            (Biome::Keep, RGB::named(LIGHT_STEEL)),
            (Biome::Mountain, RGB::named(DARK_GREY)),
            (Biome::Other, RGB::named(PINK)),
            (Biome::Plain, RGB::named(LIGHTGREEN)),
            (Biome::Road, RGB::named(SLATEGREY)),
            (Biome::Tundra, RGB::named(SNOW)),
            (Biome::Village, RGB::named(YELLOW)),
            (Biome::Wasteland, RGB::named(GREEN)),
            (Biome::Water, RGB::named(BLUE)),
        ]);

        for tile in map {
            ctx.set(tile.x, tile.y, colormap[&tile.biome], RGB::named(BLACK), to_cp437(tile.icon));
        }
    }
}