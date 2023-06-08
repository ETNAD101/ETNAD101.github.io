use yew::prelude::*;
use crate::components::menu_item_list::MenuItem;

#[derive(Properties, PartialEq)]
pub struct ItemDetailProps {
    pub item: MenuItem,
}

#[function_component(ItemDetails)]
pub fn item_details(ItemDetailProps { item }: &ItemDetailProps) -> Html {
    html! {
        <div>
            <p>{item.description.clone()}</p>
        </div>
    }
}