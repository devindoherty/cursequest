use crate::MenuItem;

#[derive(Clone, PartialEq)]
pub struct Encounter {
    pub name: String,
    pub menu: Vec<MenuItem>,
}

pub struct Quest {
    
}

struct Mob {}


impl Encounter {
    pub fn new(name: String, menu: Vec<MenuItem>) -> Encounter {
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