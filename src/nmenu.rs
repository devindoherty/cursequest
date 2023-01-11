use bracket_lib as bracket;
use bracket::prelude::*;

#[derive(Copy, Clone)]
pub struct NodeID {
    pub index: usize,
}

pub struct MenuItem {
    pub name: String,
    pub id: NodeID,
    pub children: Vec<NodeID>,
    pub selected: usize,
}

pub struct Menu {
    items: Vec<MenuItem>
}

impl Menu {
    pub fn new() -> Menu {
        Menu {items: Vec::new()}
    }

    pub fn add_item(&mut self, mut item: MenuItem) -> NodeID {
        let next_index = self.items.len();
        item.id.index = next_index;
        println!("The index of {} is now: {}", item.name, next_index);
        self.items.push(item);
        NodeID {
            index: next_index,
        }
    }

    pub fn add_child(&mut self, item_id: NodeID, child_id: NodeID) { 
        let mut item = &mut self.items[item_id.index];
        item.children.push(child_id);
    }

    pub fn remove_child(&mut self, item_id: NodeID, child_id: NodeID) {

    }

    pub fn find_child(&self, item_id: NodeID, child_id: NodeID, search: &str) {
        let item = &self.items[item_id.index];
        let child = &self.items[child_id.index];
        for child in &item.children {
            println!("{} is a child of {}", child.index, item.name);
        }
    }

    pub fn list_children(&self, item_id: NodeID) {
        let item = &self.items[item_id.index];
        for child in &item.children {
            println!("{} is a child of {}", self.items[child.index].name, item.name);
        }
    }

    pub fn terminal_draw_children(&self, item_id: NodeID) {
        let item = &self.items[item_id.index];
        println!("-------------------");
        println!("{}", item.name);
        for child in &item.children {
            println!("|-{}", self.items[child.index].name);
        }
    }

    pub fn select_child(&self) {}

    pub fn traverse(&self) {}
    
    pub fn manage(&mut self, key: VirtualKeyCode, current_id: NodeID) {
        let item = &mut self.items[current_id.index];
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => if item.selected == 0 {} else {
                item.selected -= 1;
                // println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => if item.selected >= item.children.len() - 1 {} else {
                item.selected += 1;
                // println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
            },
            VirtualKeyCode::Return => self.select_child(),
            _ => {}
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, current_id: NodeID) {
        let mut y = 45;
        for id in &self.items[current_id.index].children {
            ctx.print_color(
                1, y, 
                RGB::named(WHITE), RGB::named(BLACK), 
                self.items[id.index].name.to_string()
            );
            y += 1;
        
        }

        
    }

    pub fn ndraw(&self, ctx: &mut BTerm, current_id: NodeID) {
        let mut y = 45;
        let item = &self.items[current_id.index];
        for (pos, child) in item.children.iter().enumerate() {
             if pos == item.selected {
                ctx.print_color(
                    1, y,
                    RGB::named(BLACK), RGB::named(WHITE),
                    item.name.to_string()
                );
                y += 1;
            } else {
                ctx.print_color(
                    1, y, 
                    RGB::named(WHITE), RGB::named(BLACK), 
                    item.name.to_string()
                );
                y += 1;
            }
        }
    }
}