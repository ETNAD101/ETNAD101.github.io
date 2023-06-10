use yew::prelude::*;

use crate::components::navbar::Navbar;

/*
When you open this page, It will show a list of all the items that were added to the bill.
It gives the option to pick combined bills or seperate bills
*/

#[function_component(BillPage)]
pub fn bill_page() -> Html {
    html! {
        <>
            <Navbar/>
        </>
    }
}