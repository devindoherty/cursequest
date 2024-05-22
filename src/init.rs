use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::{Deserialize, Serialize};

// use crate::Encounter;
use crate::Art;
// use crate::Map;
use crate::dialogue::{Dialogues, DialogueID};
use crate::Flags;
use crate::scene::{Scene, SceneID};
use crate::State;
use crate::Skill;

use crate::{Menu, MenuItem};

pub fn start_menu() -> Menu {
    // Start Menu
    let start_item_one = MenuItem {
        display_name: String::from("Start"),
        display_char: '1',
    };
    let start_item_two = MenuItem {
        display_name: String::from("Continue"),
        display_char: '2',
    };
    let start_item_three = MenuItem {
        display_name: String::from("Load"),
        display_char: '3',
    };
    let start_item_four = MenuItem {
        display_name: String::from("Quit"),
        display_char: '3',
    };

    Menu {
        items: vec![start_item_one, start_item_two, start_item_three, start_item_four],
        selected: 0,
        last: Vec::new(),
    }
}

pub fn main_menu() -> Vec<MenuItem> {
    // Running Menu
    let menu_item_one = MenuItem {
        display_name: String::from("Travel"),
        display_char: '1',
    };

    let menu_item_two = MenuItem {
        display_name: String::from("Character"),
        display_char: '2',
    };

    let menu_item_three = MenuItem {
        display_name: String::from("Inventory"),
        display_char: '3',
    };

    let menu_item_four = MenuItem {
        display_name: String::from("Journal"),
        display_char: '4',
    };

    vec![
        menu_item_one,
        menu_item_two,
        menu_item_three,
        menu_item_four,
    ]
}

pub fn travel_menu() -> Vec<MenuItem> {
    // Travel Menu
    let travel_item_one = MenuItem {
        display_name: String::from("Journey On"),
        display_char: '1',
    };

    let travel_item_two = MenuItem {
        display_name: String::from("Explore"),
        display_char: '2',
    };

    let travel_item_three = MenuItem {
        display_name: String::from("Hunt"),
        display_char: '3',
    };

    let travel_item_four = MenuItem {
        display_name: String::from("Forage"),
        display_char: '3',
    };

    let travel_item_five = MenuItem {
        display_name: String::from("Camp"),
        display_char: '5',
    };

    let travel_item_six = MenuItem {
        display_name: String::from("Interact"),
        display_char: '6',
    };

    vec![
        travel_item_one,
        travel_item_two,
        travel_item_three,
        travel_item_four,
        travel_item_five,
        travel_item_six,
    ]
}

pub fn prologue() -> Scene {
    let title = String::from("Prologue");
    let text = String::from(
        "A decade ago, the Uncrowned King usurped the throne from you and banished you to a life of exile. In your wandering, you come across the tomb of a ancient king. Hewn into the stone there is a magical sword. Grasping the blade and pulling it from the stone, you fade into darkness..."
    );
    let art = Art::new("assets/title_alt2.txt", String::from("king"));
    let menu: Option<Menu> = None;
    let dialogue = None;

    Scene::new(
        title,
        text,
        art,
        true,
        menu,
        dialogue,
        SceneID { index: 0 },
    )
}

pub fn shir() -> Scene {
    let title = String::from("Roseberry, Wisewoman Apothecary");
    let text = String::new();
    let art = Art::new("assets/rose.txt", String::from("Roseberry"));
    let menu: Option<Menu> = None;
    let dialogue = None;
    
    Scene::new(
        title,
        text,
        art,
        false,
        menu,
        dialogue,
        SceneID {index: 0},
    )

}


pub fn load_dialogues() -> Dialogues {
    let dialogues = File::open("data/dialogues.yml").expect("Could not open dialogues!");
    let reader: Dialogues = serde_yaml::from_reader(dialogues).expect("Could not read dialogue values!");
    reader
}

pub fn load_flags() -> Flags {
    let flags = File::open("data/flags.yml").expect("Could not open flags!");
    let reader: Flags = serde_yaml::from_reader(flags).expect("Could not read values!");
    reader
}

pub fn _load_scenes() {

}

pub fn _load_skills() {

}

pub fn _load_abilities() {
    
}