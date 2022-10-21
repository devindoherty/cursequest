use crate::Menu;

pub struct Encounter {
    pub name: String,
    pub flavor: String,
    pub menu: Menu,
    pub art: Vec<String>, 
}

struct Mob {}

impl Encounter {
    fn new(&self, name: String, flavor: String, menu: Menu, art: Vec<String>) -> Encounter {
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