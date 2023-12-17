use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::{Deserialize, Serialize};

// use crate::Encounter;
use crate::Art;
// use crate::Map;
use crate::Dialogue;
use crate::dialogue::{DialogueItem, Link::*};
use crate::Flags;
use crate::scene::{Scene, SceneID};
use crate::State;
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
        "A decade ago, the Uncrowned King usurped the kingdom from you and banished you to a life of exile. In your wandering you have come across a legendary magical sword. But instead of granting you the power you need to slay the Uncrowned King and reclaim your throne, the sword has Cursed you. You must find a way to lift the Curse or you will surely perish, or worse. Returning to Klathia, you feel your strength falter and you lose consciousness in the wilderness.                                                                                                                                                     Press any key to continue..."
    );
    let art = Art::new("assets/king.txt", String::from("king"));
    let menu: Option<Menu> = None;
    let dialogue: Option<Dialogue> = None;

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

    let encounter_item_zero = DialogueItem {
        choice: String::from("Let us speak of other things."),
        response: String::from("You awaken in a small, rustic home. The woman seated beside you is perhaps three feet tall at most. She looks relieved when you open your eyes. \"You have stirred. Good. You were half dead when we found you. Rest now. You are safe.\""),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()
    };

    // println!("Loaded: {:?}", encounter_item_zero);

    let c0_altered = DialogueItem {
        choice: String::from("Let us speak of other things."),
        response: String::from("Roseberry tends to your wounds quietly."),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()
    };

    let encounter_item_one = DialogueItem {
        choice: String::from("Where am I?"),
        response: String::from("\"You are in the Shir Valley, in the village of Finn's Glenn.\""),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()
    };

    let encounter_item_one_alpha = DialogueItem {
        choice: String::from("This is a halfling village?"),
        response: String::from("\"It is. We are a humble folk, the Hobs of the Shir Valley. What some might say we lack in stature, we make up for in a myriad of other ways. I hope you remember our hospitality in the days ahead.\""),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()
    };

    let encounter_item_one_beta = DialogueItem {
        choice: String::from("I know of this place, and your people. [LORE 1]"),
        response: String::from("The halflings, or Hobs, of Shir Valley are an ancient people, said to be related to faeries, known to be brave and loyal. Roseberry seems surprised. \"You know the stories of my people? I am glad. Few men or dwarves, nor even elves venture here. Fewer still since the death of the last king.\""),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    }; 

    let c1b_king = DialogueItem {
        choice: String::from("King Kull is dead, then?"),
        response: String::from("The little woman sighs and pats your shoulder.\"It is said he died within the dungeons of the Uncrowned King.\" You let the truth sink in: Your father is dead. [JOURNAL UPDATED]"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
        flag_names: Some("learned_father_death".to_string()),
        ..Default::default()

    }; 

    let encounter_item_one_gamma = DialogueItem {
        choice: String::from("As you have offered your home to me, mine is open to you. [CUSTOMS 1]"),
        response: String::from("\"That is kind of you, but I fear we may find your halls less than hospitable, at the moment.\""),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    }; 

    let encounter_item_two = DialogueItem {
        choice: String::from("Who are you?"),
        response: String::from("\"Why, I'm Roseberry, the village's apothecary. This is my humble home.\" She smiles at you. \"Be welcome.\""),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        link: None,
        ..Default::default()

    };

    let encounter_item_two_alpha = DialogueItem {
        choice: String::from("Well met, Roseberry. I am Kryll, the rightful king of Klathia."),
        response: String::from("Roseberry smiles, although her face is beset with sadness. She bows her head. \"I know. I recognize your face, my king. Were that these less grim times, the people of Finn's Glenn would honor your return with celebration.\""),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: Some("king_identity_told_truth".to_string()),
        link: Some(RemoveSiblings),
        ..Default::default()

    };

    let encounter_item_two_beta = DialogueItem {
        choice: String::from("Greetings, Roseberry. I am merely a passerby of little renown."),
        response: String::from("Roseberry smiles strangely but nods, \"But of course. You are welcome all the same.\""),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        link: Some(RemoveSiblings),
        ..Default::default()

    };

    let encounter_item_three = DialogueItem {
        choice: String::from("What happened?"),
        response: String::from(
            "\"A group of hunters from my village found you collapsed in the wilderness. You were holding that.\" She points to an ancient blade among your belongings. \"I had you brought here and have nursed you back to health.\" You hear a faint whispering from the blade, as if it... calls to you."
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_three_altered = DialogueItem {
        choice: String::from("Investigate the blade further."),
        response: String::from(
            "Whispers fill the air. The blade calls to you. Roseberry shivers."
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };
    
    let encounter_item_three_a = DialogueItem {
        choice: String::from("I can... hear the blade."),
        response: String::from(
            "\"It is cursed,\" Roseberry says plainly. \"And now, as its wielder, so are you.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_three_b = DialogueItem {
        choice: String::from("Grab the blade and flourish it skillfully. [SWORD 1]"),
        response: String::from(
            "The blade seems to almost slice the air itself. Roseberry purses her lips thoughtfully. \"You could cut through a dozen armed and armored men with such a weapon as easily as I carve a cake.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        link: Some(SkillCheck {skill_name: "Swords".to_string(), difficulty: 1 }),
        ..Default::default()

    };

    let encounter_item_three_c = DialogueItem {
        choice: String::from("Hover your hand over the blade, reaching for the magic within. [SORCERY 1]"),
        response: String::from(
            "The blade radiates waves of vile black magic. Roseberry frowns, \"You sense it too. It is as I feared. This sword is cursed.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_three_d = DialogueItem {
        choice: String::from("Listen to the blade's whispers. [MARTIAL 1]"),
        response: String::from(
            "A faint voice eminates from the blade. It speaks of bloody combat. Of men marching into glorious battle. \"Those who wield me are all but invincible...\" it whispers to you. Roseberry looks from the blade to you and mutters, \"It is a cursed blade. Listen to it at your own peril.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_three_e = DialogueItem {
        choice: String::from("Envision a kingdom won through the might of this blade. [REGALITY 1]"),
        response: String::from(
            "\"The cost of thy kingdom can be tolled in blood...\" the sword whispers to you. Roseberry shakes her head. \"Be weary of such a price to pay,\" she warns."
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_three_f = DialogueItem {
        choice: String::from("Study the blade. [LORE 1]"),
        response: String::from(
            "You have read of such swords. They are forged or enchanted with malevolent spirits. The tales often do not end kindly for the wielders of such weapons. Roseberry says, \"And for good reason. The blade is evil.\""
        ),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_four = DialogueItem {
        choice: String::from("Die, witch! [ATTACK]"),
        response: String::from("Roseberry's lips quiver in shock. \"Is this truly what I am owed for saving your life?\" Grabbing the cursed blade, you strike her down with all your might. The sword laughs maliciously."),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_five = DialogueItem {
        choice: String::from("Farewell. [END CONVERSATION]"),
        response: String::from("\"Before you depart, may I ask a boon of you?\""),
        id: NodeID { index: 0 },
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let bye_no_1 = DialogueItem {
        choice: String::from("No."),
        response: String::from("The small woman who nursed you back to health looks taken aback but then nods. \"If you feel you are ready to embark, I will not stop you. But know this: The Shir Valley is relatively safe, yet the realm beyond is dangerous. The Uncrowned King holds court upon the throne. His armies are vast, and his sorceries vile...\""),
        id: NodeID::new(),
        children: vec![],
        selected: 0,        
        flag_names: None,
        ..Default::default()

    };

    let bye_no_2 = DialogueItem {
        choice: String::from("Continue..."),
        response: String::from("\"The elven queen hides in her forest realm and the dwarven king cowers in his mountain hall. The Klathian dukes and lords have all bent the knee. Any who would openly defy him, especially one as weak as you, would be doomed to fail.\""),
        id: NodeID::new(),
        children: vec![],
        selected: 0,        
        flag_names: None,
        ..Default::default()

    };

    let bye_yes = DialogueItem {
        choice: String::from("Yes."),
        response: String::from("\"Several of the hunters who rescued have since gone missing near the same place they found you. If you could find it in your heart to look for them, their families would be much relieved to have an experienced adventurer on the search,\" Roseberry explains."),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let encounter_item_cont = DialogueItem {
        choice: String::from("Continue..."),
        response: String::from("END"),
        id: NodeID {index: 0},
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let cattack = DialogueItem {
        choice: String::from("Continue..."),
        response: String::from("END"),
        id: NodeID::new(),
        children: vec![],
        selected: 0,
        flag_names: None,
        ..Default::default()

    };

    let mut dialogue = Dialogue::new();
    let c0 = dialogue.add_item(encounter_item_zero);
    let c0_altered = dialogue.add_item(c0_altered);
    let c1 = dialogue.add_item(encounter_item_one);
    let c2 = dialogue.add_item(encounter_item_two);
    let c3 = dialogue.add_item(encounter_item_three);
    let c4 = dialogue.add_item(encounter_item_four);
    
    let c5 = dialogue.add_item(encounter_item_five);
    let bye_no_1 = dialogue.add_item(bye_no_1);
    let bye_no_2 = dialogue.add_item(bye_no_2);
    let bye_yes = dialogue.add_item(bye_yes);

    let c1a = dialogue.add_item(encounter_item_one_alpha);
    let c1b = dialogue.add_item(encounter_item_one_beta);
    let c1c = dialogue.add_item(encounter_item_one_gamma);
    let c1b_king = dialogue.add_item(c1b_king);


    let c2a = dialogue.add_item(encounter_item_two_alpha);
    let c2b = dialogue.add_item(encounter_item_two_beta);
    
    let c3a = dialogue.add_item(encounter_item_three_a);
    let c3b = dialogue.add_item(encounter_item_three_b);
    let c3c = dialogue.add_item(encounter_item_three_c);
    let c3d = dialogue.add_item(encounter_item_three_d);
    let c3e = dialogue.add_item(encounter_item_three_e);
    let c3f = dialogue.add_item(encounter_item_three_f);

    let c3_altered = dialogue.add_item(encounter_item_three_altered);

    
    let cc = dialogue.add_item(encounter_item_cont);
    let cattack = dialogue.add_item(cattack);

    // dialogue.add_child(c0, c0);
    dialogue.add_child(c0, c1);
    dialogue.add_child(c0, c2);
    dialogue.add_child(c0, c3);
    dialogue.add_child(c0, c4);
    dialogue.add_child(c0, c5);

    dialogue.add_child(c1, c1a);
    dialogue.add_child(c1, c1b);
    dialogue.add_child(c1, c1c);
    dialogue.add_child(c1a, c0_altered);
    dialogue.add_child(c1b, c1b_king);
    dialogue.add_child(c1b_king, c0_altered);
    dialogue.add_child(c1c, c0);

    dialogue.add_child(c2, c2a);
    dialogue.add_child(c2, c2b);
    dialogue.add_child(c2a, c0_altered);
    dialogue.add_child(c2b, c0_altered);

    dialogue.add_child(c3, c3a); dialogue.add_child(c3a, c3_altered); 
    dialogue.add_child(c3, c3b); dialogue.add_child(c3b, c3_altered);
    dialogue.add_child(c3, c3c); dialogue.add_child(c3c, c3_altered);
    dialogue.add_child(c3, c3d); dialogue.add_child(c3d, c3_altered);
    dialogue.add_child(c3, c3e); dialogue.add_child(c3e, c3_altered);
    dialogue.add_child(c3, c3f); dialogue.add_child(c3f, c3_altered);
    
    dialogue.add_child(c3_altered, c3a);
    dialogue.add_child(c3_altered, c3b);
    dialogue.add_child(c3_altered, c3c);
    dialogue.add_child(c3_altered, c3d);
    dialogue.add_child(c3_altered, c3e);
    dialogue.add_child(c3_altered, c3f);
    dialogue.add_child(c3_altered, c0_altered);

    dialogue.add_child(c0_altered, c1);
    dialogue.add_child(c0_altered, c2);
    dialogue.add_child(c0_altered, c3);
    dialogue.add_child(c0_altered, c4);
    dialogue.add_child(c0_altered, c5);

    dialogue.add_child(c4, cattack);

    dialogue.add_child(c5, bye_yes);    
    dialogue.add_child(c5, bye_no_1);
    
    dialogue.add_child(bye_yes, cc);
    dialogue.add_child(bye_no_1, bye_no_2);
    dialogue.add_child(bye_no_2, cc);

    Scene::new(
        title,
        text,
        art,
        false,
        None,
        Some(dialogue),
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
        name: String::from("Regality"),
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

    let _intrigue = Skill {
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

    let _lore = Skill {
        name: String::from("Lore"),
        desc: String::from("Knowledge of history, sciences, and other esoterica."),
        value: 10,
        abilities: vec![],
    };
}

pub fn load_flags() -> Flags {
    let flags = File::open("data/flags.yml").expect("Could not open flags!");
    let reader: Flags = serde_yaml::from_reader(flags).expect("Could not read values!");
    reader
}

pub fn _load_scenes() {

}