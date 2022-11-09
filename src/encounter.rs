use crate::Menu;

pub struct Encounter <'a>{
    pub name: String,
    pub menu: Menu<'a>,
}


struct Mob {}


impl Encounter <'_>{
    fn new(name: String, flavor: String, menu: Menu, art: Vec<String>) -> Encounter {
        let new_encounter = Encounter {
            name: name,
            flavor: flavor,
            menu: menu,
            art: art,
        };
        new_encounter
    }
}

impl Mob {
    fn new() {

    }
}