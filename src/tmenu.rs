use bracket_lib as bracket;
use bracket::prelude::*;

pub struct ItemID {
    index: usize,
}

pub struct ItemData {
    pub name: String,
}

pub struct MenuItem {
    parent: Option<ItemID>,
    child: Option<ItemID>,

    pub data: ItemData,
}

pub struct MenuManager{
    items: Vec<MenuItem>
}

impl MenuManager {
    pub fn new() -> MenuManager {
        MenuManager {
            items: Vec::new(),
        }
    }

    pub fn new_node(&mut self, data: ItemData) -> ItemID {
        let next_index = self.items.len();
        self.items.push(MenuItem {
            parent: None,
            child: None,
            data,
        });        
        ItemID {
            index: next_index,
        }
    }
}