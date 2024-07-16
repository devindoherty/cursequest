use bracket_lib as bracket;
use bracket::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct FlagID {
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub name: String,
    pub flagged: bool,
    stage: u32,
    id: FlagID,
    pub var: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub flags: Vec<Flag>,
}

impl Flag {
    pub fn set_var(&mut self, var: String) {
        self.var = Some(var);
    }

    pub fn get_id(&self) -> FlagID {
        self.id
    }
}