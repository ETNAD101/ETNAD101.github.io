use yew::prelude::*;
use crate::components::menu_item_list::Category;
use crate::components::menu_item_list::ItemList;
use crate::components::menu_item_list::get_category;


#[function_component(App)]
pub fn app() -> Html {
    let appetizers = get_category(Category::Appetizer);
    let entrees = get_category(Category::Entree);
    let drinks = get_category(Category::Drink);
    let desserts = get_category(Category::Dessert);
    html! {
        <>
            <h1>{"Drinks"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={drinks}/>
            </div>

            <h1>{"Appetizers"}</h1>
            <div class={"ItemCategory"}> 
                <ItemList items={appetizers}/>
            </div>

            <h1>{"Entrees"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={entrees}/>
            </div>

            <h1>{"Desserts"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={desserts}/>
            </div>
        </>
    }
}