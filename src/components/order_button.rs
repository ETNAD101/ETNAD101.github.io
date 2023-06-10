use yew::prelude::*;
use gloo::console::log;

use crate::backend::menu::MenuItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: MenuItem,
}


#[function_component(OrderButton)]
pub fn order_button(Props {item}: &Props) -> Html {

    let on_click = Callback::from(move |_| {
        log!("Order Placed");
    });

    html! {
        <button class={"orderButton"} onclick={on_click}>{"Order"}</button>
    }
}