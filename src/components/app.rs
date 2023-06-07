use yew::prelude::*;
use crate::components::menu_item_list::Category;
use crate::components::menu_item_list::ItemList;
use crate::components::menu_item_list::get_category;


#[function_component(App)]
pub fn app() -> Html {
    let appetizers = get_category(Category::Appetizer);
    html! {
        <>
            <ItemList items={appetizers}/>
        </>
    }
}