use crate::Menu;

pub struct Encounter {
    pub name: String,
    pub menu: Menu,
}


struct Mob {}


impl Encounter <'_>{
    fn new(name: String, flavor: String, menu: Menu, art: Vec<String>) -> Encounter {
        Encounter {
            name,
            menu,
        }
    }
}

impl Mob {
    fn new() {

    }
}