

// Different "modes" for the game
#[derive(Debug, PartialEq)]
pub enum RunMode {
    Start,
    Storytelling,
    Prompting,
    Travelling,
    Fighting {combat: Combat},
    Updating,
}

#[derive(Debug, PartialEq)]
pub enum Combat {PlayerTurn, MobTurn}
