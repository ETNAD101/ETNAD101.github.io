use yew::prelude::*;
use crate::backend::menu::MenuItem;

#[derive(Properties, PartialEq)]
pub struct ItemDetailProps {
    pub item: MenuItem,
}

#[function_component(ItemDetails)]
pub fn item_details(ItemDetailProps { item }: &ItemDetailProps) -> Html {
    html! {
        <div>
            <p class={"description"}>{item.description.clone()}</p>
        </div>
    }
}