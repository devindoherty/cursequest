use bracket_lib as bracket;
use bracket::prelude::*;

// use crate::State;
// use crate::Player;
// use crate::Map;
// use crate::map::Biome;


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

    fn select(&self) {
        let item = &self.items[self.selected].display_name;
        println!("{}", item);
        if item.contains("[END CONVERSATION") {
            println!("YOoooooOOOo GET ME OUTA HERE!");
        }
    }

    pub fn manage(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => if self.selected == 0 {} else {
                self.selected -= 1;
                // println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => if self.selected >= self.items.len() - 1 {} else {
                self.selected += 1;
                // println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            VirtualKeyCode::Return => self.select(),
            _ => {}
        }
    }

    // Stack ops
    // pub fn push_menu(&mut self, previous: Menu) {
    //     self.last.push(previous);
    // }

    // pub fn pop_menu(&mut self,) -> Option<Menu> {
    //     self.last.pop()
    // }

    // pub fn push_item(&mut self, item: MenuItem) {
    //     self.items.push(item)
    // }

    // pub fn pop_item(&mut self) -> Option<MenuItem>{
    //     self.items.pop()
    // }

}


