use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::components::navbar::Navbar;
use crate::components::clear_cart::ClearCart;
use crate::components::seperate_bills::SeperateBills;
use crate::components::combined_bill::CombinedBill;
use crate::stores::bill_state_store::BillState;

/*
When you open this page, It will show a list of all the items that were added to the bill.
It gives the option to pick combined bills or seperate bills
*/

#[function_component(BillPage)]
pub fn bill_page() -> Html {
    let (bill, dispatch) = use_store::<BillState>();
    dispatch.reduce_mut(|_bill| html!{<combined_bill/>});
    
    let c_dispatch = dispatch.clone();
    let combined_click = Callback::from(move |_| {
        c_dispatch.reduce_mut(|bill| bill.bill_toggle = 0);
    });

    let seperate_click = Callback::from(move |_| {
        dispatch.reduce_mut(|bill| bill.bill_toggle = 1);
    });

    let display_bill: Html;

    if bill.bill_toggle == 0 {
        display_bill = html! {
            <CombinedBill/>
        };
    } else {
        display_bill = html! {
            <SeperateBills/>
        };
    }

    html! {
        <>
            <Navbar/>
            {display_bill}
            <ClearCart/>
            <button onclick={seperate_click}>{"Seperate Bills"}</button>
            <button onclick={combined_click}>{"Combined Bill"}</button>
        </>
    }
}