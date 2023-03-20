use bracket::prelude::*;
use bracket_lib as bracket;

use crate::State;

#[derive(Copy, Clone, Debug)]
pub struct NodeID {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct DialogueItem {
    pub name: String,
    pub id: NodeID,
    pub children: Vec<NodeID>,
    pub selected: usize,
}

#[derive(Clone, Debug)]
pub struct Dialogue {
    pub items: Vec<DialogueItem>,
    current: NodeID,
}

impl Dialogue {
    pub fn new() -> Dialogue {
        Dialogue {
            items: Vec::new(),
            current: NodeID { index: 0 },
        }
    }

    pub fn add_item(&mut self, mut item: DialogueItem) -> NodeID {
        let next_index = self.items.len();
        item.id.index = next_index;
        println!("Diaglogue: The index of {} is now: {}", item.name, next_index);
        self.items.push(item);
        NodeID { index: next_index }
    }

    pub fn add_child(&mut self, item_id: NodeID, child_id: NodeID) {
        let item = &mut self.items[item_id.index];
        item.children.push(child_id);
    }

    pub fn remove_child(&mut self, item_id: NodeID, child_id: NodeID) {
        let item = &mut self.items[item_id.index];
        let child = &mut self.items[child_id.index];
        // item.children.remove(child); // TODO! Rework remove
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
            println!(
                "{} is a child of {}",
                self.items[child.index].name, item.name
            );
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

    pub fn select_child(&mut self) {
        let item = &self.items[self.current.index];
        let child = &self.items[item.selected];
        println!("Selected: {}", child.name);
        self.traverse(child.id); // Work in progress
    }

    pub fn traverse(&mut self, item_id: NodeID) {
        self.current = item_id;
        let item = &self.items[item_id.index];
        self.change_text();
        println!("Traversed to: {}", item.name);

        self.terminal_draw_children(item_id);
    }

    pub fn change_text(&self) -> String {
        let item = &self.items[self.current.index];
        let mut new_main = String::new();
        if item.name == "Who are you?" {
            println!("Change Text Triggered");
            new_main = String::from("I am Rosebery the Wisewoman.");
            return new_main;
        }
        new_main
    }

    pub fn end_dialogue(&self) {

    }

    pub fn manage(&mut self, key: VirtualKeyCode) {
        let item = &mut self.items[self.current.index];
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                if item.selected == 0 {
                    () // Do nothing, top of dialogue choices
                } else {
                    item.selected -= 1;
                    // println!("{} selected: {}", item.name, item.selected);
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                if item.selected >= item.children.len() - 1 {
                    (); // Do Nothing, bottom of dialogue choices
                } else {
                    item.selected += 1;
                    // println!("{} selected: {}", item.name, item.selected);
                    // println!("Selected Menu Item is: {}", self.items[self.selected].display_name);
                }
            }
            VirtualKeyCode::Return => self.select_child(),
            _ => {}
        }
    }

    // Rendering dialogue options to choice selection
    pub fn draw(&self, ctx: &mut BTerm) {
        let mut y = 50;
        let item = &self.items[self.current.index];
        let item_with_name = String::from("Kryll:") + &item.name;
        ctx.print_color(
            3,
            49,
            RGB::named(BLACK),
            RGB::named(WHITE),
            item_with_name,
        );
        for (pos, child) in item.children.iter().enumerate() {
            if pos == item.selected {
                ctx.print_color(
                    3,
                    y,
                    RGB::named(BLACK),
                    RGB::named(WHITE),
                    self.items[child.index].name.to_string(),
                );
                y += 1;
            } else {
                ctx.print_color(
                    3,
                    y,
                    RGB::named(WHITE),
                    RGB::named(BLACK),
                    self.items[child.index].name.to_string(),
                );
                y += 1;
            }
        }
    }
}
