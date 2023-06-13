use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::cart_store::CartStore;
use crate::backend::cart::CartItem;

#[function_component(SeperateBills)]
pub fn seperate_bill() -> Html {
    let (cart, _dispatch) = use_store::<CartStore>();
    
    let mut sorted_carts: Vec<Html> = vec![];

    for person in cart.people.clone() {
        let items: Vec<&CartItem> = cart.cart_items.iter().filter(|item| item.person == person).clone().collect();

        let mut total_f: f64 = 0.0;

        let listed_items: Html = items.iter().map(|item: &&CartItem| {
            total_f += item.item.price as f64;
            
            html!{
                <p class={"billedItem"}>{format!("{} ⋅ ${}", item.item.name, item.item.price)}</p>
            }
        }).collect();

        let total = html! {
            <p>{format!("Total: ${:.2}", total_f)}</p>
        };

        let bill: Html = html! {
            <div class={"bill"}>
                <h1 class={"billTitle"}>{person}</h1>
                {listed_items}
                {total}
            </div>
        };

        sorted_carts.push(bill)
    }

    html! {
        <div class={"seperateBills"}>
            {for sorted_carts}
        </div>
    }
}

    