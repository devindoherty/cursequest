pub struct Encounter {
    pub name: String,
    // pub enemies: Vec<Mob>,
    pub flavor: String,
    pub kind: Kind,
    pub art: i32, 
}

struct Mob {}

pub enum Kind {Fight, Merchant, Random, Story}

impl Encounter {

    fn new() {
        
    }
}

impl Mob {
    fn new() {

    }
}