use std::collections::HashMap;

#[derive(Clone)]
pub struct NodeID {
    pub index: usize,
}

#[derive(Clone)]
pub struct MenuItem {
    pub name: String,
    pub id: NodeID,
    pub edges: Option<HashMap<NodeID, MenuItem>>,
}

pub struct Menu {
    items: Vec<MenuItem>
}

impl Menu {
    pub fn new() -> Menu {
        Menu {items: Vec::new()}
    }

    pub fn add_item(&mut self, mut item: MenuItem) -> NodeID {
        let next_index = self.items.len();
        item.id.index = next_index;
        self.items.push(item.clone());
        println!("The index of {} is now: {}", item.name, next_index);
        
        NodeID {
            index: next_index,
        }
    }

    fn connect_edge(&self, item: NodeID) {

    }
}