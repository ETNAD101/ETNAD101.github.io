use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Category {
    Appetizer,
    Entree,
    Drink,
    Dessert,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub category: Category,
    pub name: String,
    pub price: f32,
    pub description: String,
}

impl MenuItem {
    fn new(category: Category, name: String, price: f32) -> Self {
        MenuItem {
            category: category,
            name: name,
            price: price,
            description: " ".to_string(),
        }
    }
}

pub fn get_category(category: Category) -> Vec<MenuItem> {
    match category {
        Category::Appetizer => {
            
        }
    }
    vec![
        // Appetizers
        MenuItem::new(Category::Appetizer, "Guacamole & Chips".to_string(), 5.99),
        MenuItem::new(Category::Appetizer, "Buffalo Wings".to_string(), 5.99),
        MenuItem::new(Category::Appetizer, "Brushetta".to_string(), 4.99),
        MenuItem::new(Category::Appetizer, "Hummus".to_string(), 3.99),
        MenuItem::new(Category::Appetizer, "Escargot".to_string(), 8.99),
        MenuItem::new(Category::Appetizer, "Shrimp".to_string(), 6.99),
    
        MenuItem::new(Category::Entree, "Baked Chicken Tacos".to_string(), 12.99),
        MenuItem::new(Category::Entree, "Bolognese".to_string(), 14.99),
        MenuItem::new(Category::Entree, "White Pizza".to_string(), 8.99),
        MenuItem::new(Category::Entree, "Vegetarian Chili".to_string(), 10.99),
        MenuItem::new(Category::Entree, "Broccoli Cheddar Soup".to_string(), 9.99),
        MenuItem::new(Category::Entree, "Roasted Turkey Breast".to_string(), 16.99),
        MenuItem::new(Category::Entree, "Salmon With Garlic Cream Sauce".to_string(), 11.99),
    
        MenuItem::new(Category::Drink, "Miami Vice".to_string(), 4.99),
        MenuItem::new(Category::Drink, "Shirley Temple".to_string(), 3.99),
        MenuItem::new(Category::Drink, "Chocolate Milk".to_string(), 2.99),
        MenuItem::new(Category::Drink, "Mojito".to_string(), 4.99),
        MenuItem::new(Category::Drink, "Daiquiri".to_string(), 3.99),
        MenuItem::new(Category::Drink, "Margarita".to_string(), 4.99),
    
        MenuItem::new(Category::Dessert, "Chocolate Ice Cream".to_string(), 3.99),
        MenuItem::new(Category::Dessert, "Homemade Brownie".to_string(), 2.99),
        MenuItem::new(Category::Dessert, "Apple Pie".to_string(), 3.99),
        MenuItem::new(Category::Dessert, "Lava Cake".to_string(), 4.99),
        MenuItem::new(Category::Dessert, "Mohci".to_string(), 1.99),
        MenuItem::new(Category::Dessert, "Tiramisu".to_string(), 4.99),
    ]
}

#[derive(Properties, PartialEq)]
pub struct MenuListProps {
    pub items: Vec<MenuItem>,
}

#[function_component(ItemList)]
pub fn itemlist(MenuListProps { items }: &MenuListProps) -> Html {
    items.iter().map(|item| html!{
        <p>{format!("{}: {}", item.name, item.price)}</p>
    }).collect()
}
