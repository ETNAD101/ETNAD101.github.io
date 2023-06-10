use yew::prelude::*;
use crate::backend::menu::MenuItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: MenuItem,
}

#[function_component(ItemDetails)]
pub fn item_details(Props { item }: &Props) -> Html {
    html! {
        <div>
            <p class={"description"}>{item.description.clone()}</p>
        </div>
    }
}