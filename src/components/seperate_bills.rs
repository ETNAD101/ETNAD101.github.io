use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::cart_store::CartStore;
use crate::backend::cart::CartItem;

#[function_component(SeperateBills)]
pub fn carts() -> Html {
    let (cart, _dispatch) = use_store::<CartStore>();
    
    let mut sorted_carts: Vec<Html> = vec![];

    for person in cart.people.clone() {
        let items: Vec<&CartItem> = cart.cart_items.iter().filter(|item| item.person == person).clone().collect();

        let listed_items: Html = items.iter().map(|item: &&CartItem| {
            html!{
                <p>{format!("{} ⋅ ${}", item.item.name, item.item.price)}</p>
            }
        }).collect();

        let bill: Html = html! {
            <div class={"bill"}>
                <h1>{person}</h1>
                {listed_items}
            </div>
        };

        sorted_carts.push(bill)
    }

    html! {
        <>
            {for sorted_carts}
        </>
    }
}

    