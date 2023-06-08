use yew::prelude::*;
use crate::components::menu_item_list::MenuItem;
use crate::components::menu_item_list::Category;
use crate::components::menu_item_list::ItemList;
use crate::components::menu_item_list::get_category_from_menu;
use crate::components::item_details::ItemDetails;


#[function_component(App)]
pub fn app() -> Html {
    let appetizers = get_category_from_menu(Category::Appetizer);
    let entrees = get_category_from_menu(Category::Entree);
    let drinks = get_category_from_menu(Category::Drink);
    let desserts = get_category_from_menu(Category::Dessert);

    let selected_item = use_state(|| None);

    let on_item_select = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: MenuItem| {
            selected_item.set(Some(item));
        })
    };

    let details = selected_item.as_ref().map(|item| html! {
        <ItemDetails item={item.clone()}/>
    });

    html! {
        <div>
            <h1>{"Drinks"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={drinks} on_click={on_item_select.clone()} />
            </div>

            <h1>{"Appetizers"}</h1>
            <div class={"ItemCategory"}> 
                <ItemList items={appetizers} on_click={on_item_select.clone()} />
            </div>

            <h1>{"Entrees"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={entrees} on_click={on_item_select.clone()} />
            </div>

            <h1>{"Desserts"}</h1>
            <div class={"ItemCategory"}>  
                <ItemList items={desserts} on_click={on_item_select.clone()} />
            </div>
            {for details}
            <div>
                <p>{"Clicked"}</p>
            </div>
        </div>

    }
}