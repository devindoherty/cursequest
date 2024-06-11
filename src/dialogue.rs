use std::collections::HashMap;

use bracket::prelude::*;
use bracket_lib as bracket;

use serde::{Deserialize, Serialize};

use crate::{init, FlagID, RunMode, Skill, State, Statistics};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Status {
    visible: bool,
    selectable: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
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
    name: String,
    id: DialogueID,
    choice: String,
    response: String,
    status: Option<Status>,
    skillcheck: Option<(String, i32)>,
    flags: Option<Vec<FlagID>>,
    children: Vec<DialogueID>,
}

impl Dialogue {
    pub fn new(name: String, choice: String, response: String) -> Dialogue {
        Dialogue {
            name,
            id: DialogueID::new(),
            choice,
            response,
            status: None,
            skillcheck: None,
            flags: None,
            children: Vec::new(),
        }
    }

    pub fn get_response(&self) -> &String {
        &self.response
    }

    pub fn set_id(&mut self, id: DialogueID) {
        self.id = id;
    }

    pub fn get_id(&self) -> DialogueID {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialogues {
    pub items: Vec<Dialogue>,
    current: DialogueID,
    previous: DialogueID,
    selected: usize,
}

impl Dialogues {
    pub fn new(items: Vec<Dialogue>, current: DialogueID) -> Self {
        Dialogues {
            items,
            current,
            previous: DialogueID {index: 0},
            selected: 0,
        }
    }

    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.items.push(dialogue);
    }

    pub fn register_dialogue(&mut self, mut dialogue: Dialogue) -> DialogueID {
        let next_index = self.items.len();
        dialogue.id.index = next_index;
        self.items.push(dialogue);
        DialogueID { index: next_index }
    }

    pub fn link_child(&mut self, parent_id: DialogueID, child_id: DialogueID) {
        let parent = &mut self.items[parent_id.index];
        parent.children.push(child_id);
    }

    pub fn remove_child(&mut self, item_id: DialogueID, child_id: DialogueID) {
        self.items.retain(|item| item.id != child_id);
    }
    
    pub fn get_dialogue_id_with_name(&self, name: &str) -> DialogueID {
        for dialogue in &self.items {
            if dialogue.name == name {
                return dialogue.id;
            }
        }
        DialogueID {index: 0}
    }

    pub fn get_dialogue(&mut self, id: DialogueID) -> &mut Dialogue {
        &mut self.items[id.index]
    }

    pub fn get_current_dialogue(&self) -> &Dialogue {
        &self.items[self.current.index]
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

    fn get_current_dialogue_id(&mut self) -> DialogueID {
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
        let selection = item.children[self.selected];

        self.change_dialogue(selection);

        self.traverse(selection);
    }

    fn change_dialogue(&mut self, id: DialogueID) {
        let dialogue = self.get_dialogue(id);
        if dialogue.response.contains("$") {
            dialogue.response = dialogue.response.replace("$pc_price", "blood price");
        }
    }

    fn traverse(&mut self, item_id: DialogueID) {
        self.previous = self.current; 
        self.current = item_id;
    }

    pub fn set_dialogue_var() {
        
    }


    pub fn end_dialogue(&self, gs: &mut State) {
       
    }

    pub fn manage(&mut self, key: VirtualKeyCode) {
        let item = &self.items[self.current.index];
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
                self.select_child();
                self.selected = 0;
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
