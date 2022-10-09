use bracket_lib as bracket;
use bracket::prelude::*;

pub struct Menu {
    pub items: Vec<MenuItem>,
}

pub struct MenuItem {
    pub display_name: String,
    pub display_char: char,
}

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu { items: Vec::new() };
        menu
    }

    pub fn push(&mut self, item: MenuItem) {
        self.items.push(item)
    }

    pub fn pop(&mut self) -> Option<MenuItem>{
        self.items.pop()
    }

    pub fn draw(&mut self, ctx: &mut BTerm) {
        // for item in &self.items {
        //     println!("{}", item.display_name);
        let mut y = 42;
        for item in &self.items {
            ctx.print_color(
                1, y, 
                RGB::named(WHITE), RGB::named(BLACK), 
                item.display_name.to_string()
            );
            y += 1;
        }
    }

    pub fn manage(&mut self, ctx: &mut BTerm) {
        let mut selected_item = &self.items[0].display_name;
    }

}

impl MenuItem {
    
    pub fn new(name: String, character: char) -> MenuItem {
        let menu_item = MenuItem {
            display_name: name,
            display_char: character,
        };

        menu_item
    }
}
