use menu::Category;

/*
Creative Name: ____
-Minimum 10  items
-3 categories(Appetizers, Entrees, Drinks), At least 6 options each section
*/
pub mod menu;

#[macro_use]
extern crate lazy_static;

fn main() {
    let ref menu_items: Vec<menu::MenuItem> = *menu::MENU;
    for item in menu_items.iter() {
        match item.category {
            Category::Appetizer => println!("{}", item.name),

            _ => println!("Uhhh"),
        }
    }
    
}