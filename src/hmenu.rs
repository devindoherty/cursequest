use std::collections::HashMap;

pub struct NodeID {
    index: usize,
}

pub struct MenuItem {
    name: String,
    edges: HashMap<NodeID, MenuItem>,
}

pub struct Menu {
    items: Vec<MenuItem>
}

impl Menu {
    pub fn new() -> Menu {
        Menu {items: Vec::new()}
    }

    fn add_item(&self, item: MenuItem) -> NodeID {
        let next_index = self.items.len();
        NodeID {
            index: next_index,
        }
    }
}