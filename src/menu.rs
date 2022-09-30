use bracket_lib as bracket;
use bracket::prelude::*;

pub struct Menu {
    pub items: Vec<MenuItem>,
}

pub struct MenuItem {
    pub display_name: String,
}

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu { items: Vec::new() };
        menu
    }

    pub fn update(&mut self, item: MenuItem) {
        self.items.pop();
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
}

