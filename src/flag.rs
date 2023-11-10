use bracket_lib as bracket;
use bracket::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct FlagID {
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub name: String,
    pub flagged: bool,
    pub stage: u32,
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub flags: Vec<Flag>,
}