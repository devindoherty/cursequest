use bracket_lib as bracket;
use bracket::prelude::*;

use crate::State;
use crate::Player;
use crate::Map;
use crate::map::Biome;

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub selected: usize,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub display_name: String,
    pub display_char: char,
}

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu { 
            items: Vec::new(),
            selected: 0,
        };
        menu
    }

    pub fn push(&mut self, item: MenuItem) {
        self.items.push(item)
    }

    pub fn pop(&mut self) -> Option<MenuItem>{
        self.items.pop()
    }

    pub fn draw(&mut self, ctx: &mut BTerm) {
        let mut y = 45;
        for (pos, item) in self.items.iter().enumerate() {
             if pos == self.selected {
                ctx.print_color(
                    1, y,
                    RGB::named(BLACK), RGB::named(WHITE),
                    item.display_name.to_string()
                );
                y += 1;
            } else {
                ctx.print_color(
                    1, y, 
                    RGB::named(WHITE), RGB::named(BLACK), 
                    item.display_name.to_string()
                );
                y += 1;
            }
        }
    }

    pub fn manage(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up => if self.selected == 0 {} else {
                self.selected -= 1;
                println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            VirtualKeyCode::Down => if self.selected >= self.items.len() - 1 {} else {
                self.selected += 1;
                println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            _ => {}
        }
    }

    pub fn alter(&mut self, player: &Player, map: &Map) {
        let map = &map.atlas;
        
        let city = MenuItem {
            display_name: String::from("Enter City"),
            display_char: 'c',
        };

        for tile in map {
            if player.x == tile.x && player.y == tile.y {
                if tile.biome == Biome::City {
                    self.push(city.clone());
                }
            }
        }
        
    }

}

impl MenuItem {
    pub fn new(name: String, character: char) -> MenuItem {
        let menu_item = MenuItem {
            display_name: name,
            display_char: character,
        };
        menu_item
    }
}
