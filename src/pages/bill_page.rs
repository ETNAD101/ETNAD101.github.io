use yew::prelude::*;
use yewdux::prelude::*;

use crate::backend::cart::CartItem;
use crate::components::navbar::Navbar;
use crate::components::clear_cart::ClearCart;
use crate::stores::cart_store::CartStore;

/*
When you open this page, It will show a list of all the items that were added to the bill.
It gives the option to pick combined bills or seperate bills
*/

#[function_component(BillPage)]
pub fn bill_page() -> Html {
    let (cart, _dispatch) = use_store::<CartStore>();
    
    let temp: Html = cart.cart_items.iter().map(|item: &CartItem| {
        html!{
        <div class={"item"}>
            <p>{format!("{}: {} ⋅ ${}", item.person, item.item.name, item.item.price)}</p>
        </div>
        }
    }).collect();

    html! {
        <>
            <Navbar/>
            {temp}
            <ClearCart/>
        </>
    }
}