use bracket::prelude::*;
use bracket_lib as bracket;



#[derive(Copy, Clone, Debug)]
pub struct NodeID {
    pub index: usize,
}

impl NodeID {
    pub fn new() -> NodeID {
        NodeID {index: 0}
    }
}

#[derive(Clone, Debug)]
pub struct DialogueItem {
    pub id: NodeID,
    pub response: String,
    pub choice: String,
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
        println!("Diaglogue: The index of {} is now: {}", item.choice, next_index);
        self.items.push(item);
        NodeID { index: next_index }
    }

    pub fn add_child(&mut self, item_id: NodeID, child_id: NodeID) {
        let item = &mut self.items[item_id.index];
        item.children.push(child_id);
    }

    pub fn remove_child(&mut self, item_id: NodeID, child_id: NodeID) {
        let _item = &mut self.items[item_id.index];
        let _child = &mut self.items[child_id.index];
        // item.children.remove(child); // TODO! Rework remove
    }

    pub fn find_child(&self, item_id: NodeID, child_id: NodeID, _search: &str) {
        let item = &self.items[item_id.index];
        let _child = &self.items[child_id.index];
        for child in &item.children {
            println!("{} is a child of {}", child.index, item.choice);
        }
    }

    pub fn list_children(&self, item_id: NodeID) {
        let item = &self.items[item_id.index];
        for child in &item.children {
            println!(
                "{} is a child of {}",
                self.items[child.index].choice, item.choice
            );
        }
    }

    pub fn terminal_draw_children(&self, item_id: NodeID) {
        let item = &self.items[item_id.index];
        println!("-------------------");
        println!("{}", item.choice);
        for child in &item.children {
            println!("|-{}", self.items[child.index].choice);
        }
    }

    pub fn select_child(&mut self) {
        let item = &self.items[self.current.index];
        let child = &self.items[item.selected];
        println!("Selected: {}", child.choice);
        self.traverse(child.id); // Work in progress
    }

    pub fn traverse(&mut self, item_id: NodeID) {
        self.current = item_id;
        let item = &self.items[item_id.index];
        self.change_text();
        println!("Traversed to: {}", item.choice);

        self.terminal_draw_children(item_id);
    }

    pub fn change_text(&self) -> String {
        let item = &self.items[self.current.index];
        let mut new_main = String::new();
        if item.choice == "Who are you?" {
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
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                if item.selected >= item.children.len() -1 {
                    (); // Do Nothing, bottom of dialogue choices
                } else {
                    item.selected += 1;
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
        let display = &item.response;
        // ctx.print_color(
        //     3,
        //     49,
        //     RGB::named(BLACK),
        //     RGB::named(WHITE),
        //     display.to_string(),
        // );
        for (pos, child) in item.children.iter().enumerate() {
            if pos == item.selected {
                ctx.print_color(
                    3,
                    y,
                    RGB::named(BLACK),
                    RGB::named(WHITE),
                    self.items[child.index].choice.to_string(),
                );
                y += 1;
            } else {
                ctx.print_color(
                    3,
                    y,
                    RGB::named(WHITE),
                    RGB::named(BLACK),
                    self.items[child.index].choice.to_string(),
                );
                y += 1;
            }
        }
    }
}
