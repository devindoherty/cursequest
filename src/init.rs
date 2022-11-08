// use crate::Encounter;
use crate::Map;
use crate::{Menu, MenuItem};

pub fn start_menu() -> Menu {
    
    // Start Menu
    let start_item_one = MenuItem{
        display_name: String::from("Start"),
        display_char: '1',
    };
    let start_item_two = MenuItem{
        display_name: String::from("Continue"),
        display_char: '2',
    };
    let start_item_three = MenuItem{
        display_name: String::from("Quit"),
        display_char: '3'
    };

    Menu {
        items: vec![start_item_one, start_item_two, start_item_three],
        selected: 0,
        last: Vec::new(),
    }
}

pub fn main_menu() -> Vec<MenuItem> {
    // Running Menu
    let menu_item_one = MenuItem {
        display_name: String::from("Travel"),
        display_char: '1'
    };

    let menu_item_two = MenuItem {
        display_name: String::from("Character"),
        display_char: '2'
    };

    let menu_item_three = MenuItem {
        display_name: String::from("Inventory"),
        display_char: '3'
    };

    let menu_item_four = MenuItem {
        display_name: String::from("Journal"),
        display_char: '4'
    };

    let items = vec![menu_item_one, menu_item_two, menu_item_three, menu_item_four];
    items
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

    let items = vec![
        travel_item_one, 
        travel_item_two, 
        travel_item_three, 
        travel_item_four, 
        travel_item_five,
        travel_item_six
    ];
    items
}

// pub fn prologue() -> Scene {
//     Scene{}
// }

// fn init_encounters() {
//     // let main_quest_menu = Menu{
//     //     items: Vec::new(),
//     //     selected: 0,

//     // };

//     // let main_quest = Encounter {
//     //     name: String::from("The Curse Quest"),
//     //     flavor: String::from("The village elder, a wizened crone of a halfling woman informs you that you are lucky to be alive. You have been cursed."),
//     //     menu: main_quest_menu,
//     //     art: Vec::new(), 
//     // };
// }