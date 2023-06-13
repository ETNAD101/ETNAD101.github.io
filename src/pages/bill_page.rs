use yew::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::clear_cart::ClearCart;
use crate::components::seperate_bills::SeperateBills;
use crate::components::combined_bill::CombinedBill;

/*
When you open this page, It will show a list of all the items that were added to the bill.
It gives the option to pick combined bills or seperate bills
*/

#[function_component(BillPage)]
pub fn bill_page() -> Html {
    html! {
        <>
            <Navbar/>
            <CombinedBill />
            <SeperateBills />
            <ClearCart/>
        </>
    }
}