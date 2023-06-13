use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::cart_store::CartStore;

#[function_component(CombinedBill)]
pub fn combined_bill() -> Html {
    let (cart, _dispatch) = use_store::<CartStore>();

    let mut total_f: f64 = 0.0;
    let items = cart.cart_items.clone();

    let listed_items: Html = items.iter().map(|item| {
        total_f += item.item.price as f64;
        
        html!{
            <p class={"billedItem"}>{format!("{} ⋅ ${}", item.item.name, item.item.price)}</p>
        }
    }).collect();

    let total = html! {
        <p>{format!("Total: ${:.2}", total_f)}</p>
    };

    html! {
        <div class={"bill"}>
            <h1 class={"billTitle"}>{"Total"}</h1>
            {listed_items}
            {total}
        </div>
    }
}

    