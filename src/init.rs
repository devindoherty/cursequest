// use crate::Encounter;
use crate::Art;
// use crate::Map;
use crate::Dialogue;
use crate::dialogue::DialogueItem;
use crate::scene::{Flag, Scene, SceneID};
use crate::NodeID;
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
        display_name: String::from("Quit"),
        display_char: '3',
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
    let main = String::from(
        "A decade ago, the Uncrowned King usurped the title from you and banished you to a life of exile. In your wandering you have come across a legendary magical sword. But instead of granting you the power you need to slay the Uncrowned King and reclaim your throne, the sword has Cursed you. You must find a way to lift the Curse or you will fall under the evil sword's malicious control.
        
        Press any key to continue..."
    );
    let art = Art::new("assets/king.txt", String::from("king"));
    let menu: Option<Menu> = None;
    let dialogue: Option<Dialogue> = None;
    let flags = vec![Flag {name: "press_any_key".to_string(), flagged: true}]; 

    Scene::new(
        title,
        main,
        art,
        true,
        menu,
        dialogue,
        Some(flags),
        SceneID { index: 0 },
    )
}

pub fn shir() -> Scene {
    let title = String::from("Awakening");
    let main = String::from(
        "You have stirred. Good. You were half dead when we found you. Rest now. You are safe.",
    );
    let art = Art::new("assets/rose.txt", String::from("Roseberry"));

    let encounter_item_zero = DialogueItem {
        choice: String::from("ROOT"),
        response: String::from("Choose thy response..."),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
    };

    let encounter_item_one = DialogueItem {
        choice: String::from("Where am I?"),
        response: String::from("\"You are in the Shir Valley, in the illage of Finn's Glenn.\""),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
    };

    let encounter_item_two = DialogueItem {
        choice: String::from("Who are you?"),
        response: String::from("\"Why, I'm Roseberry, of course. This is my humble home.\""),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
    };

    let encounter_item_three = DialogueItem {
        choice: String::from("What happened?"),
        response: String::from(
            "\"A group of farmers found you collapsed in the wilderness. 
            You were holding this...\" She points out an ancient blade among your belongings. 
            \"I had you brought here and have nursed you back to health.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
    };

    let encounter_item_four = DialogueItem {
        choice: String::from("Farewell [END CONVERSATION]"),
        response: String::from("Test"),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
    };

    let encounter_item_cont = DialogueItem {
        choice: String::from("Continue..."),
        response: String::from("Continued"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
    };

    

    let mut dialogue = Dialogue::new();
    let c0 = dialogue.add_item(encounter_item_zero);
    let c1 = dialogue.add_item(encounter_item_one);
    let c2 = dialogue.add_item(encounter_item_two);
    let c3 = dialogue.add_item(encounter_item_three);
    let c4 = dialogue.add_item(encounter_item_four);
    let cc = dialogue.add_item(encounter_item_cont);
    

    dialogue.add_child(c0, c1);
    dialogue.add_child(c0, c2);
    dialogue.add_child(c0, c3);
    dialogue.add_child(c0, c4);
    dialogue.add_child(c1, cc);
    dialogue.add_child(c2, cc);
    dialogue.add_child(c3, cc);
    dialogue.add_child(c4, cc);

    // dialogue.add_child(c1, c2);
    // dialogue.add_child(c1, c3);
    // dialogue.add_child(c1, c4);

    // dialogue.add_child(c2, c1);
    // dialogue.add_child(c2, c3);
    // dialogue.add_child(c2, c4);

    // dialogue.add_child(c3, c1);
    // dialogue.add_child(c3, c2);
    // dialogue.add_child(c3, c4);
    
    Scene::new(
        title,
        main,
        art,
        false,
        None,
        Some(dialogue),
        None,
        SceneID { index: 0 },
    )
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
        desc: String::from("Measure of kingly authority and right-to-rule."),
        value: 10,
        abilities: vec![],
    };

    let _customs = Skill {
        name: String::from("Customs"),
        desc: String::from("Ability to socialize with different classes of society."),
        value: 0,
        abilities: vec![],
    };

    let _shadowplay = Skill {
        name: String::from("Intrigue"),
        desc: String::from("Familiarity with the shadows, subterfuge, and trickery."),
        value: 10,
        abilities: vec![],
    };

    let _survival = Skill {
        name: String::from("Survival"),
        desc: String::from("Capacity to endure the harsh wildnerness."),
        value: 10,
        abilities: vec![],
    };
}
