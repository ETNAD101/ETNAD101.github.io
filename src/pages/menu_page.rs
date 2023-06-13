use yew::prelude::*;
use crate::backend::category::Category;
use crate::components::item_list::ItemList;
use crate::components::navbar::Navbar;
use crate::backend::menu::get_category_from_menu;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category: String,
}

#[function_component(MenuPage)]
pub fn menu(Props {category}: &Props) -> Html {
    let c = match category.as_str() {
        "Appetizer" => Category::Appetizer,
        "Entree" => Category::Entree,
        "Drink" => Category::Drink,
        "Dessert" => Category::Dessert,
        _ => Category::Appetizer,
    };

    let food_list = get_category_from_menu(c);

    html! {
        <>
            <Navbar/>

            <div class={"menu"}>
                <h1 class={"categoryHeader"}>{category}</h1>
                <div class={"ItemCategory"}>  
                    <ItemList items={food_list} />
                </div>
            </div>
        </>
    }
}