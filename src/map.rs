use bracket_lib as bracket;
use bracket::prelude::*;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Map {
    pub atlas: Vec<MapTile>,
}

pub struct MapTile {
    icon: char,
    pub desc: String,
    pub x: i32,
    pub y: i32,
    pub biome: Biome,
    _id: i32,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Biome {
    City,
    Desert,
    Dungeon,
    Enclave,
    Farm,
    Forest,
    Jungle,
    Grassland,
    Hall,
    Keep,
    Marsh,
    Mountain,
    Other,
    Plain,
    Poi,
    Road,
    Temple,
    Tundra,
    Village,
    Wasteland,
    Water,
}

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
                    'x' => String::from("Blasted wasteland."),
                    'y' | 'Y' => String::from("Cursed jungle."), 
                    'o' | ';' => String::from("Marshy swamps."),
                    'V' | 'H' => String::from("A provincial village"),
                    '#' => String::from("Fertile farmland."),
                    '=' | '/' | '\\' | '-' | '|' => String::from("A traveller's road."),
                    'D' => String::from("A dangerous dungeon."),
                    '*' => String::from("Windswept tundra."),
                    '%' => String::from("Sandy desert."),
                    'T' => String::from("Temple of the Magi"),
                    _ => String::from("Klathian wilderness."),
                }, 
                x: x,
                y: y,
                _id: i,
                biome: match c {
                    'x' => Biome::Wasteland,
                    'o' | ';' => Biome::Marsh,
                    'V' | 'H' => Biome::Village,
                    'M' => Biome::Hall,
                    'K' => Biome::Keep,
                    'f' | 'F' => Biome::Forest,
                    'Y' | 'y' => Biome::Jungle,
                    'E' => Biome::Enclave,
                    'D' => Biome::Dungeon,
                    'C' => Biome::City,
                    'A' | '^' => Biome::Mountain,
                    '~' => Biome::Water,
                    '=' | '/' | '\\' | '-' | '|' => Biome::Road,
                    '%' => Biome::Desert,
                    '#' => Biome::Farm,
                    '*' => Biome::Tundra,
                    '.' => Biome::Plain,
                    ',' => Biome::Grassland,
                    'T' => Biome::Temple,
                    '!' => Biome::Poi,
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
            (Biome::City, RGB::named(YELLOW)),
            (Biome::Desert, RGB::named(SANDYBROWN)),
            (Biome::Dungeon, RGB::named(PURPLE)),
            (Biome::Enclave, RGB::named(GREEN)),
            (Biome::Farm, RGB::named(WHEAT3)),
            (Biome::Forest, RGB::named(FORESTGREEN)),
            (Biome::Jungle, RGB::named(VIOLET)),
            (Biome::Grassland, RGB::named(LAWNGREEN)),
            (Biome::Hall, RGB::named(STEELBLUE)),
            (Biome::Keep, RGB::named(LIGHT_STEEL)),
            (Biome::Marsh, RGB::named(DARK_GREEN)),
            (Biome::Mountain, RGB::named(DARK_GREY)),
            (Biome::Other, RGB::named(BLUE)),
            (Biome::Plain, RGB::named(LIGHTGREEN)),
            (Biome::Poi, RGB::named(RED)),
            (Biome::Road, RGB::named(SLATEGREY)),
            (Biome::Temple, RGB::named(PINK)),
            (Biome::Tundra, RGB::named(SNOW)),
            (Biome::Village, RGB::named(GOLD)),
            (Biome::Wasteland, RGB::named(OLIVE)),
            (Biome::Water, RGB::named(BLUE)),
        ]);

        for tile in map {
            ctx.set(tile.x, tile.y, colormap[&tile.biome], RGB::named(BLACK), to_cp437(tile.icon));
        }
    }
}

impl MapTile {
    // fn new(map: &Map, icon: char, desc: String, x: i32, y: i32, biome: Biome) -> MapTile {
    //     let new_tile = MapTile {
    //         icon: icon,
    //         pub desc: desc,
    //         pub x: x,
    //         pub y: y,
    //         biome: biome,
    //     };
    //     new_tile
    // }

    // fn alter() {
        
    // }
}