// use crate::Encounter;
use crate::Art;
// use crate::Map;
use crate::{Menu, MenuItem};
use crate::Scene;
use crate::Skill;
use crate::NodeID;
use crate::dialogue;
use crate::State;


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
    let start_item_three = MenuItem {
        display_name: String::from("New Menu Test"),
        display_char: '3',
    };
    let start_item_four = MenuItem{
        display_name: String::from("Quit"),
        display_char: '3'
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
    let encounter: Option<Menu> = None;
    let nencounter: Option<NodeID> = None;
    Scene::new(title, main, art, encounter, nencounter)
}

pub fn shir() -> Scene {
    let title = String::from("Roseberry the Healer");
    let main = String::from("You have stirred. Good. You were half dead when we found you. Rest now. You are safe.");
    let art = Art::new("assets/rose.txt", String::from("Roseberry the Healer"));

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

    let encounter_item_four = MenuItem {
        display_name: String::from("Farewell [END CONVERSATION]"),
        display_char: '4',
    };

    let encounter_items = vec![
        encounter_item_one, 
        encounter_item_two, 
        encounter_item_three, 
        encounter_item_four
    ];

    let encounter = Menu {
        items: encounter_items,
        selected: 0,
        last: Vec::new(),
    };

    let nencounter: Option<NodeID> = None;

    Scene::new(title, main, art, Some(encounter), nencounter)
}

pub fn nshir(gs: &mut State) {
 
    let title = String::from("Rosebery the Healer");
    let main = String::from("You have stirred. Good. You were half dead when we found you. Rest now. You are safe.");
    let art = Art::new("assets/rose.txt", String::from("Roseberry the Healer"));

    let encounter_item_one = dialogue::MenuItem {
        name: String::from("Where am I?"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
    };

    let encounter_item_two = dialogue::MenuItem {
        name: String::from("Who are you?"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
    };

    let encounter_item_three = dialogue::MenuItem {
        name: String::from("What happened?"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
    };

    let encounter_item_four = dialogue::MenuItem {
        name: String::from("Farewell [END CONVERSATION]"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
    };

    gs.dialogue.add_item(encounter_item_one);
    gs.dialogue.add_item(encounter_item_two);
    gs.dialogue.add_item(encounter_item_three);
    gs.dialogue.add_item(encounter_item_four);
    
    let encounter: Option<Menu> = None;
    let nencounter: Option<NodeID> = None;
    Scene::new(title, main, art, encounter, nencounter); 
}

pub fn _skills() {
    
    let _sword = Skill {
        name: String::from("Sword"),
        desc: String::from("Mastery of the Cursed Blade."),
        value: 10,
        abilities: vec![],
    };

    let _sorcery = Skill {
        name: String::from("Sorcery"),
        desc: String::from("Knowledge of the secrets of magic."),
        value: 10,
        abilities: vec![],
    };

    let _martial = Skill {
        name: String::from("Martial"),
        desc: String::from("Propensity for physical violence and related tactics."),
        value: 0,
        abilities: vec![],
    };

    let _sovereignty = Skill {
        name: String::from("Sovereignty"),
        desc: String::from("Measure of kingly authority and charisma."),
        value:10,
        abilities:vec![],
    };

    let _customs = Skill {
        name: String::from("Customs"),
        desc: String::from("Ability to socialize with different classes of society."),
        value: 0,
        abilities:vec![],
    };

    let _subterfuge = Skill {
        name: String::from("Intrigue"),
        desc: String::from("Familiarity with the shadows, subterfuge, and trickery."),
        value: 10,
        abilities: vec![],
    };

    let _survival = Skill {
        name: String::from("Survival"),
        desc: String::from("Capacity to endure the harsh wildnerness and bowmanship."),
        value: 10,
        abilities: vec![],
    };
}