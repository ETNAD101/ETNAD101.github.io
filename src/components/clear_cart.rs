use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::cart_store::CartStore;

#[function_component(ClearCart)]
pub fn order_button() -> Html {
    let dispatch = Dispatch::<CartStore>::new();

    let on_click = Callback::from(move |_| {
        dispatch.reduce_mut(|cart| cart.cart_items.clear());
    });

    html! {
        <button class={"clearCart"} onclick={on_click}>{"Clear Cart"}</button>
    }
}