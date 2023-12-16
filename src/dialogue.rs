use bracket::prelude::*;
use bracket_lib as bracket;

use crate::{init, State, Skill, Statistics, RunMode};

#[derive(Clone, Debug, Default)]
pub enum Link {
    #[default]
    Remove,
    RemoveSiblings,
    Move,
    Change {change_text: String},
    CheckSkill{skill_name: String, difficulty: i32},
    CheckStat{stat_name: String, difficulty: i32},
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NodeID {
    pub index: usize,
}

impl NodeID {
    pub fn new() -> Self {
        NodeID {index: 0}
    }
}

#[derive(Clone, Debug, Default)]
pub struct DialogueItem <> {
    pub id: NodeID,
    pub response: String,
    pub choice: String,
    pub children: Vec<NodeID>,
    pub selected: usize,
    pub flag_names: Option<String>,
    pub link: Option<Link>, 
}


#[derive(Clone, Debug)]
pub struct Dialogue {
    pub items: Vec<DialogueItem>,
    pub current: NodeID,
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
        // println!("Diaglogue: The index of {} is now: {}", item.choice, next_index);
        self.items.push(item);
        NodeID { index: next_index }
    }

    pub fn add_child(&mut self, item_id: NodeID, child_id: NodeID) {
        let item = &mut self.items[item_id.index];
        item.children.push(child_id);
    }

    pub fn remove_child(&mut self) {
        todo!();
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

    pub fn remove_siblings(&mut self) {
        let selection = self.current_selection();
        let child = self.items[selection.index].id;
        
        let item = &mut self.items[self.current.index];
        
        item.children = Vec::new();
        item.children.push(child);
    }

    fn change(&self, change_text: &String) {
        todo!();
    }

    fn current_selection(&self) -> NodeID {
        let item = &self.items[self.current.index];
        let selection = &item.children[item.selected];
        *selection
    }

    fn terminal_draw_children(&self, item_id: NodeID) {
        let item = &self.items[item_id.index];
        println!("-------------------");
        println!("{}", item.choice);
        for child in &item.children {
            println!("|-{}", self.items[child.index].choice);
        }
    }

    fn select_child(&mut self) {
        let item = &self.items[self.current.index];
        let selection = &item.children[item.selected];
        let child = &self.items[selection.index];
                
        self.traverse(child.id);
    }

    fn check_links(&mut self) {
        let item = &self.items[self.current.index];
        let selection = &item.children[item.selected];
        let child = &self.items[selection.index];
        let link = &child.link;

        if link.is_some() {
            match link.as_ref().unwrap() {
                Link::Remove => self.remove_child(),
                Link::RemoveSiblings => self.remove_siblings(),
                Link::Change {change_text} => self.change(change_text),

                _  => todo!(),
            }
        }
    }

    fn traverse(&mut self, item_id: NodeID) {
        self.current = item_id;
        let item = &self.items[item_id.index];
        // self.change_text();
        println!("Traversed to: {}", item.choice);
        self.terminal_draw_children(item_id);
    }

    pub fn end_dialogue(&self, gs: &mut State) {
       
    }

    pub fn get_current_item(&self) -> &DialogueItem {
        &self.items[self.current.index]
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
                if item.selected >= item.children.len() - 1 {
                    (); // Do Nothing, bottom of dialogue choices
                } else {
                    item.selected += 1;
                }
            }
            VirtualKeyCode::Return => {
                self.check_links();
                self.select_child();
            }
            _ => {}
        }
    }

    // Rendering dialogue options to choice selection
    pub fn draw(&self, ctx: &mut BTerm) {
        let mut y = 50;
        let item = &self.items[self.current.index];
        let display = &item.response;
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
