use yew::prelude::*;
use gloo::console::log;

#[function_component(OrderButton)]
pub fn order_button() -> Html {

    let on_click = Callback::from(move |_| {
        log!("Order Placed")
    });

    html! {
        <button onclick={on_click}>{"Order"}</button>
    }
}