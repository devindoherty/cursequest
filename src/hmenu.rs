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