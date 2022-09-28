pub struct Encounter {
    pub name: String,
    pub enemies: Vec<Mob>,
    pub flavor: String,
    pub kind: Kind, 
}

struct Mob {}

enum Kind {Fight, Merchant, Random}

impl Encounter {

    fn new() {
        
    }
}