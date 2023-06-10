use yew::prelude::*;
use crate::backend::category::Category;
use crate::components::item_list::ItemList;
use crate::components::navbar::Navbar;
use crate::backend::menu::get_category_from_menu;

#[function_component(MenuPage)]
pub fn menu() -> Html {
    let appetizers = get_category_from_menu(Category::Appetizer);
    let entrees = get_category_from_menu(Category::Entree);
    let drinks = get_category_from_menu(Category::Drink);
    let desserts = get_category_from_menu(Category::Dessert);
    
    html! {
        <>
            <Navbar/>

            <div class={"menu"}>
                <h1 class={"categoryHeader"}>{"Drinks"}</h1>
                <div class={"ItemCategory"}>  
                    <ItemList items={drinks} />
                </div>

                <h1 class={"categoryHeader"}>{"Appetizers"}</h1>
                <div class={"ItemCategory"}> 
                    <ItemList items={appetizers} />
                </div>

                <h1 class={"categoryHeader"}>{"Entrees"}</h1>
                <div class={"ItemCategory"}>  
                    <ItemList items={entrees} />
                </div>

                <h1 class={"categoryHeader"}>{"Desserts"}</h1>
                <div class={"ItemCategory"}>  
                    <ItemList items={desserts} />
                </div>
            </div>
        </>
    }
}