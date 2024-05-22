#[derive(Clone, Debug)]
pub struct Statistics {
    pub grace: i32, // Agility and dexterity; change to hit with all attacks
    pub might: i32, // Constitution and strength; damage for all attacks
    pub mind: i32,  // Wisdom and Intellect; magical aptitude and potency
    pub soul: i32,  // Force of personality and ego; interpersonal skills, regen, and divine luck
}

#[derive(Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub desc: String,
    pub value: i32,
 //   pub abilities: Vec<Ability>,
}

pub struct Talent {
    pub name: String,
    pub desc: String,
    pub value: i32,
}

