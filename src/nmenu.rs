use std::collections::HashMap;

#[derive(Clone)]
pub struct NodeID {
    pub index: usize,
}

#[derive(Clone)]
pub struct MenuItem {
    pub name: String,
    pub id: NodeID,
    pub children: Vec<NodeID>,
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
        println!("The index of {} is now: {}", item.name, next_index);
        self.items.push(item);
        NodeID {
            index: next_index,
        }
    }

    pub fn add_children(&mut self, item_id: NodeID, child_id: NodeID) {
        let item = &mut self.items[item_id.index];
        item.children.push(child_id);        
        for child in &item.children {
            println!("{} is a child of {}", child.index, item.name);
        }
    }

    pub fn remove_children(&mut self, item_id: NodeID, child_id: NodeID) {

    }

    pub fn find_item(&self, item: NodeID) {

    }

    pub fn select_child() {}

    pub fn traverse(&self) {}

    pub fn draw() {}

}