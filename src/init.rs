// use crate::Encounter;
use crate::Art;
// use crate::Map;
use crate::{Menu, MenuItem};
use crate::Scene;
use crate::Encounter;


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

    vec![menu_item_one, menu_item_two, menu_item_three, menu_item_four]

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
        travel_item_six
    ]
}

pub fn prologue() -> Scene {
    let title = String::from("Prologue");
    let main = String::from("A decade ago, the Uncrowned King usurped the title from you and banished you to a life of exile. In your wandering you have come across a legendary magical sword. But instead of granting you the power you need to slay the Uncrowned King and reclaim your throne, the sword has Cursed you. You must find a way to lift the Curse or you will fall under the evil sword's malicious control...");
    let art = Art::new("assets/king.txt", String::from("king"));
    let encounter: Option<Encounter> = None;

    Scene::new(title, main, art, encounter)
}

pub fn shir() -> Scene {
    let title = String::from("Elder Rose");
    let main = String::from("You have stirred. Good. You were half dead when we found you. Rest now. You are safe.");
    let art = Art::new("assets/elder.txt", String::from("Elder Rose"));

    let encounter_item_one = MenuItem {
        display_name: String::from("Where am I?"),
        display_char: '1',
    };

    let encounter_item_two = MenuItem {
        display_name: String::from("Who are you?"),
        display_char: '2',
    };

    let encounter_item_three = MenuItem {
        display_name: String::from("What happened?"),
        display_char: '3',
    };

    let encounter_items = vec![encounter_item_one, encounter_item_two, encounter_item_three];

    let encounter = Encounter::new(String::from("Elder Rose"), encounter_items);

    Scene::new(title, main, art, Some(encounter))
}

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