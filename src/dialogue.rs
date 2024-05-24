use bracket::prelude::*;
use bracket_lib as bracket;

use serde::{Deserialize, Serialize};

use crate::{init, FlagID, RunMode, Skill, State, Statistics};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Status {
    visible: bool,
    selectable: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct DialogueID {
    pub index: usize,
}

impl DialogueID {
    pub fn new() -> Self {
        DialogueID {index: 0}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialogue {
    id: DialogueID,
    choice: String,
    pub response: String,
    status: Option<Status>,
    skillcheck: Option<(String, i32)>,
    flags: Option<Vec<FlagID>>,
    children: Vec<DialogueID>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialogues {
    pub items: Vec<Dialogue>,
    pub current: DialogueID,
    pub previous: DialogueID,
    selected: usize,
}

impl Dialogues {
    pub fn new(items: Vec<Dialogue>) -> Self {
        Dialogues {
            items,
            current: DialogueID {index: 0},
            previous: DialogueID {index: 0},
            selected: 0,
        }
    }

    pub fn register_dialogue(&mut self, mut dialogue: Dialogue) -> DialogueID {
        let next_index = self.items.len();
        dialogue.id.index = next_index;
        // println!("Diaglogue: The index of {} is now: {}", item.choice, next_index);
        self.items.push(dialogue);
        DialogueID { index: next_index }
    }

    pub fn add_child(&mut self, item_id: DialogueID, child_id: DialogueID) {
        let item = &mut self.items[item_id.index];
        item.children.push(child_id);
    }

    pub fn remove_child(&mut self, item_id: DialogueID, child_id: DialogueID) {
        self.items.retain(|item| item.id != child_id);
    }
    
    pub fn find_child(&self, item_id: DialogueID, child_id: DialogueID, _search: &str) {
        let item = &self.items[item_id.index];
        let _child = &self.items[child_id.index];
        for child in &item.children {
            println!("{} is a child of {}", child.index, item.choice);
        }
    }

    pub fn list_children(&self, item_id: DialogueID) {
        let item = &self.items[item_id.index];
        for child in &item.children {
            println!(
                "{} is a child of {}",
                self.items[child.index].choice, item.choice
            );
        }
    }

    fn remove_siblings(&mut self) { // Need to fix for top down usage!
        let mut parent = &mut self.items[self.previous.index];
        let mut children = &mut parent.children;
        let child = self.current;
        self.selected = 0;
        children.retain(|&x| x == child);
    }

    fn change(&self, change_text: String) {
        todo!();
    }

    fn current_selection(&self) -> DialogueID {
        let item = &self.items[self.current.index];
        let selection = &item.children[self.selected];
        *selection
    }

    fn previous_selection(&self) -> DialogueID {
        let item = &self.items[self.previous.index];
        let selection = &item.children[self.selected];
        *selection
    }

    fn get_current_dialogue(&mut self) -> DialogueID {
        self.current
    }

    fn terminal_draw_children(&self, item_id: DialogueID) {
        let item = &self.items[item_id.index];
        println!("-------------------");
        println!("{}", item.choice);
        for child in &item.children {
            println!("|-{}", self.items[child.index].choice);
        }
    }

    fn select_child(&mut self) {
        let item = &self.items[self.current.index];
        let selection = &item.children[self.selected];
        let child = &self.items[selection.index];
                
        self.traverse(child.id);
    }

    fn traverse(&mut self, item_id: DialogueID) {
        self.previous = self.current; 
        println!("{:?}", self.items[self.previous.index]);
        self.current = item_id;
        let item = &self.items[item_id.index];
        println!("Traversed to: {}", item.choice);
        self.terminal_draw_children(item_id);
    }

    pub fn end_dialogue(&self, gs: &mut State) {
       
    }

    pub fn manage(&mut self, key: VirtualKeyCode) {
        let item = &mut self.items[self.current.index];
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                if self.selected == 0 {
                    () // Do nothing, top of dialogue choices
                } else {
                    self.selected -= 1;
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                if self.selected >= item.children.len() - 1 {
                    (); // Do Nothing, bottom of dialogue choices
                } else {
                    self.selected += 1;
                }
            }
            VirtualKeyCode::Return => {
                // self.check_links();
                self.select_child();
            }
            _ => {}
        }
        self.selected = 0;
    }

    // Rendering dialogue options to choice selection
    pub fn draw(&self, ctx: &mut BTerm) {
        let mut y = 50;
        let item = &self.items[self.current.index];
        let display = &item.response;
        for (pos, child) in item.children.iter().enumerate() {
            // Currently selected
            if pos == self.selected {
                ctx.print_color(
                    3,
                    y,
                    RGB::named(BLACK),
                    RGB::named(WHITE),
                    self.items[child.index].choice.to_string(),
                );
                y += 1;
            } 
            else {
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
