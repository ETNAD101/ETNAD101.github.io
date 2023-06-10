use yew::prelude::*;
use crate::backend::route::Route;
use yew_router::prelude::use_navigator;

use crate::backend::menu::MenuItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: MenuItem,
}


#[function_component(OrderButton)]
pub fn order_button(Props {item}: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = item.id.clone();

    let on_click = Callback::from(move |_| {
        navigator.push(&Route::Select { item_id: id });
    });

    html! {
        <button class={"orderButton"} onclick={on_click}>{"Order"}</button>
    }
}