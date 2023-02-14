use bracket_lib as bracket;
use bracket::prelude::*;

use crate::State;
use crate::mode;
// use crate::Player;
// use crate::Map;
// use crate::map::Biome;
use crate::RunMode;

#[derive(Clone)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub selected: usize,
    pub last: Vec<Self>,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    // parent: Option<ItemID>,
    // child: Option<ItemID>,
    pub display_name: String,
    pub display_char: char,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, selected: usize, last: Vec<Self>) -> Menu {
        Menu {items, selected, last}
    }
    
    pub fn switch(&mut self, items: Vec<MenuItem>) -> Menu {
        self.last.push(self.clone());
        let tmp_last = self.last.clone();
        Menu {
            items,
            selected: 0,
            last: tmp_last,
        }
    }

    pub fn restore(&mut self) -> Menu {
        let last = self.last.pop();
        match last {
            None => Menu {items: Vec::new(), selected: 0, last: Vec::new()},
            Some(menu) => menu
        }
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
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => if self.selected == 0 {} 
            else {
                self.selected -= 1;
            },
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => if self.selected >= self.items.len() - 1 {} 
            else {
                self.selected += 1;
            },
            VirtualKeyCode::Return => self.select(),
            _ => {}
        }
    }

    fn select(&self) {
        let item = &self.items[self.selected].display_name;
        println!("{}", item);
        if item.contains("Start") {
            // gs.run_mode = RunMode::Prologue;
        }
    }
}


