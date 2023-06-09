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
    pub id: u16,
    pub category: Category,
    pub name: String,
    pub price: f32,
    pub image: String,
    pub description: String,
}

impl MenuItem {
    fn new(category: Category, id: u16, name: String, image: String, price: f32, description: String) -> Self {
        MenuItem {
            id: id,
            category: category,
            name: name,
            price: price,
            image: image,
            description: description,
        }
    }
}

fn make_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::new(Category::Appetizer, 1, "Guacamole & Chips".to_string(), "/assets/img/Guacamole.jpg".to_string(), 5.99, "Homemade with fresh avacados and chips".to_string()),
        MenuItem::new(Category::Appetizer, 11, "Broccoli Cheddar Soup".to_string(), "/assets/img/Broccoli-Cheddar.jpg".to_string(), 5.99, "Caramelized onion and garlic-based, thickened with a butter and flour mixture then simmered with veggie stock, broccoli, and carrots".to_string()),
        MenuItem::new(Category::Appetizer, 3, "Bruschetta".to_string(), "/assets/img/Bruschetta.jpg".to_string(), 4.99, "Grilled bread rubbed with garlic and topped with olive oil, salt, tomato, and cheese".to_string()),
        MenuItem::new(Category::Appetizer, 4, "Hummus".to_string(), "/assets/img/Hummus.jpg".to_string(), 3.99, "Fresh hummus served with naan".to_string()),
        MenuItem::new(Category::Appetizer, 5, "Escargot".to_string(), "/assets/img/Escargot.jpg".to_string(), 8.99, "French dish made from cooked land snails, served in garlic butter sauce".to_string()),
        MenuItem::new(Category::Appetizer, 6, "Shrimp".to_string(), "/assets/img/Shrimp.jpg".to_string(), 6.99, "3 Grilled shrimp skewer".to_string()),

        MenuItem::new(Category::Entree, 7, "Baked Chicken Tacos".to_string(), "/assets/img/Chicken-Tacos.jpg".to_string(), 12.99, "3 juicy baked chicken tacos, with your choice of hard or soft shell".to_string()),
        MenuItem::new(Category::Entree, 8, "Bolognese".to_string(), "/assets/img/Bolognese.jpg".to_string(), 14.99, "Meat sauce on the pasta of your choice".to_string()),
        MenuItem::new(Category::Entree, 9, "White Pizza".to_string(), "/assets/img/White-Pizza.jpg".to_string(), 8.99, "pizza topped with olive oil, garlic, cheese, salt".to_string()),
        MenuItem::new(Category::Entree, 10, "Vegetarian Chili".to_string(), "/assets/img/Veg-Chili.jpg".to_string(), 10.99, "Mushrooms, bell peppers, onions, tomatoes, lentils, sweetcorn ".to_string()),
        MenuItem::new(Category::Entree, 2, "Buffalo Wings".to_string(), "/assets/img/Buffalo-Wings.jpg".to_string(), 9.99, "Wings doused in hot buffalo sauce, with a side of celery".to_string()),
        MenuItem::new(Category::Entree, 12, "Roasted Turkey Breast".to_string(), "/assets/img/Turkey.jpg".to_string(), 16.99, "Juicy oven roasted turkey breast".to_string()),
        MenuItem::new(Category::Entree, 13, "Salmon With Garlic Cream Sauce".to_string(), "/assets/img/Salmon.jpg".to_string(), 11.99, "Salmon served with garlic cream sauce and a side of asparagus".to_string()),

        MenuItem::new(Category::Drink, 14, "Miami Vice".to_string(), "/assets/img/Miami-Vice.jpg".to_string(), 4.99, "1/2 strawberry daquiri, 1/2 pina colada".to_string()),
        MenuItem::new(Category::Drink, 15, "Shirley Temple".to_string(), "/assets/img/Shirley.jpg".to_string(), 3.99, "Sprite, Grenadine and orange juice, with a maraschino cherry".to_string()),
        MenuItem::new(Category::Drink, 16, "Chocolate Milk".to_string(), "/assets/img/Chocolate-Milk.jpg".to_string(), 2.99, "Classic chocolate milk".to_string()),
        MenuItem::new(Category::Drink, 17, "Mojito".to_string(), "/assets/img/Mojito.jpg".to_string(), 4.99, "White rum, sugar, lime juice, soda water, and mint".to_string()),
        MenuItem::new(Category::Drink, 18, "Banana Daiquiri".to_string(), "/assets/img/Banana-Daquiri.jpg".to_string(), 3.99, "1 banana, coconut milk, lime juice".to_string()),
        MenuItem::new(Category::Drink, 19, "Margarita".to_string(), "/assets/img/Margarita.jpg".to_string(), 4.99, "Tequila, triple sec, and lime juice".to_string()),

        MenuItem::new(Category::Dessert, 20, "Chocolate Ice Cream".to_string(), "/assets/img/Chocolate-Ice.jpg".to_string(), 3.99, "Classic chocolate ice cream".to_string()),
        MenuItem::new(Category::Dessert, 21, "Homemade Brownie".to_string(), "/assets/img/Brownie.jpg".to_string(), 2.99, "Fresh homemade brownies".to_string()),
        MenuItem::new(Category::Dessert, 22, "Apple Pie".to_string(), "/assets/img/Apple-Pie.jpg".to_string(), 3.99, "Homemade apple pie, with an option to have whipped cream or ice cream on top".to_string()),
        MenuItem::new(Category::Dessert, 23, "Lava Cake".to_string(), "/assets/img/lava-Cake.jpg".to_string(), 4.99, "Chocolate cake with a molten chocolate core".to_string()),
        MenuItem::new(Category::Dessert, 24, "Mohci".to_string(), "/assets/img/Mochi.jpg".to_string(), 1.99, "Japanese rice cake with creamy filling".to_string()),
        MenuItem::new(Category::Dessert, 25, "Tiramisu".to_string(), "/assets/img/Tiramisu.jpg".to_string(), 4.99, "Coffee flavoured dessert made of ladyfingers dipped in coffee, layered with a whipped mixture of eggs, sugar, and mascarpone cheese, flavoured with cocoa".to_string()),
    ]
}


pub fn get_category_from_menu(category: Category) -> Vec<MenuItem> {
    let menu = make_menu();

    menu.into_iter().filter(|item| item.category == category).clone().collect()
}

pub fn get_item_from_id(id: u16) -> Option<MenuItem> {
    let menu = make_menu();

    for item in menu {
        if item.id == id {
            return Some(item);
        }
    }
    return None;
}


#[derive(Properties, PartialEq)]
pub struct MenuListProps {
    pub items: Vec<MenuItem>,
}

#[function_component(ItemList)]
pub fn itemlist(MenuListProps { items }: &MenuListProps) -> Html {

    items.iter().map(|item: &MenuItem| {
        html!{
        <div class={item.category.to_string()}>
            <p>{format!("{} ⋅ ${}", item.name, item.price)}</p>
            <a href={format!("/description/{}", item.id)}><img src={item.image.clone()}/></a>
            <br/>
            <button>{"Order"}</button>
        </div>
        }
    }).collect()
}
