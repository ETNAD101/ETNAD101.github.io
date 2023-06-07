use yew::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Category {
    Appetizer,
    Entree,
    Drink,
    Dessert,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Appetizer => write!(f, "Appetizer"),
            Category::Entree => write!(f, "Entree"),
            Category::Drink => write!(f, "Drink"),
            Category::Dessert => write!(f, "Dessert"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub category: Category,
    pub name: String,
    pub price: f32,
    pub image: String,
    pub description: String,
}

impl MenuItem {
    fn new(category: Category, name: String, image: String, price: f32) -> Self {
        MenuItem {
            category: category,
            name: name,
            price: price,
            image: image,
            description: " ".to_string(),
        }
    }
}

pub fn get_category(category: Category) -> Vec<MenuItem> {
    match category {
        Category::Appetizer => return vec![
            MenuItem::new(Category::Appetizer, "Guacamole & Chips".to_string(), "/assets/img/Guacamole.jpg".to_string(), 5.99),
            MenuItem::new(Category::Appetizer, "Buffalo Wings".to_string(), "/assets/img/Buffalo-Wings.jpg".to_string(), 5.99),
            MenuItem::new(Category::Appetizer, "Bruschetta".to_string(), "/assets/img/Bruschetta.jpg".to_string(), 4.99),
            MenuItem::new(Category::Appetizer, "Hummus".to_string(), "/assets/img/Hummus.jpg".to_string(), 3.99),
            MenuItem::new(Category::Appetizer, "Escargot".to_string(), "/assets/img/Escargot.jpg".to_string(), 8.99),
            MenuItem::new(Category::Appetizer, "Shrimp".to_string(), "/assets/img/Shrimp.jpg".to_string(), 6.99),
        ],

        Category::Entree => return vec![
            MenuItem::new(Category::Entree, "Baked Chicken Tacos".to_string(), "/assets/img/Chicken-Tacos.jpg".to_string(), 12.99),
            MenuItem::new(Category::Entree, "Bolognese".to_string(), "/assets/img/Bolognese.jpg".to_string(), 14.99),
            MenuItem::new(Category::Entree, "White Pizza".to_string(), "/assets/img/White-Pizza.jpg".to_string(), 8.99),
            MenuItem::new(Category::Entree, "Vegetarian Chili".to_string(), "/assets/img/Veg-Chili.jpg".to_string(), 10.99),
            MenuItem::new(Category::Entree, "Broccoli Cheddar Soup".to_string(), "/assets/img/Broccoli-Cheddar.jpg".to_string(), 9.99),
            MenuItem::new(Category::Entree, "Roasted Turkey Breast".to_string(), "/assets/img/Turkey.jpg".to_string(), 16.99),
            MenuItem::new(Category::Entree, "Salmon With Garlic Cream Sauce".to_string(), "/assets/img/Salmon.jpg".to_string(), 11.99),
        ],

        Category::Dessert => return vec![
            MenuItem::new(Category::Drink, "Miami Vice".to_string(), "/assets/img/Miami-Vice.jpg".to_string(), 4.99),
            MenuItem::new(Category::Drink, "Shirley Temple".to_string(), "/assets/img/Shirley.jpg".to_string(), 3.99),
            MenuItem::new(Category::Drink, "Chocolate Milk".to_string(), "/assets/img/Chocolate-Milk.jpg".to_string(), 2.99),
            MenuItem::new(Category::Drink, "Mojito".to_string(), "/assets/img/Mojito.jpg".to_string(), 4.99),
            MenuItem::new(Category::Drink, "Banana Daiquiri".to_string(), "/assets/img/Banana-Daquiri.jpg".to_string(), 3.99),
            MenuItem::new(Category::Drink, "Margarita".to_string(), "/assets/img/Margarita.jpg".to_string(), 4.99),
        ],

        Category::Drink => vec![
            MenuItem::new(Category::Dessert, "Chocolate Ice Cream".to_string(), "/assets/img/Chocolate-Ice.jpg".to_string(), 3.99),
            MenuItem::new(Category::Dessert, "Homemade Brownie".to_string(), "/assets/img/Brownie.jpg".to_string(), 2.99),
            MenuItem::new(Category::Dessert, "Apple Pie".to_string(), "/assets/img/Apple-Pie.jpg".to_string(), 3.99),
            MenuItem::new(Category::Dessert, "Lava Cake".to_string(), "/assets/img/lava-Cake.jpg".to_string(), 4.99),
            MenuItem::new(Category::Dessert, "Mohci".to_string(), "/assets/img/Mochi.jpg".to_string(), 1.99),
            MenuItem::new(Category::Dessert, "Tiramisu".to_string(), "/assets/img/Tiramisu.jpg".to_string(), 4.99),
        ],
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuListProps {
    pub items: Vec<MenuItem>,
}

#[function_component(ItemList)]
pub fn itemlist(MenuListProps { items }: &MenuListProps) -> Html {
    items.iter().map(|item: &MenuItem| html!{
        <div class={item.category.to_string()}>
            <p>{format!("{}: {}", item.name, item.price)}</p>
            <img src={item.image.clone()}/>
        </div>
    }).collect()
}
