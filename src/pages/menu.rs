use yew::prelude::*;
use crate::components::menu_item_list::Category;
use crate::components::menu_item_list::ItemList;
use crate::components::menu_item_list::get_category_from_menu;

#[function_component(Menu)]
pub fn menu() -> Html {
    let appetizers = get_category_from_menu(Category::Appetizer);
    let entrees = get_category_from_menu(Category::Entree);
    let drinks = get_category_from_menu(Category::Drink);
    let desserts = get_category_from_menu(Category::Dessert);
    
    html! {
        <div class={"menu"}>
            <h1>{"Drinks"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={drinks} />
            </div>

            <h1>{"Appetizers"}</h1>
            <div class={"ItemCategory"}> 
                <ItemList items={appetizers} />
            </div>

            <h1>{"Entrees"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={entrees} />
            </div>

            <h1>{"Desserts"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={desserts} />
            </div>
        </div>
    }
}