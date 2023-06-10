use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(BillPage)]
pub fn bill_page() -> Html {
    html! {
        <>
            <Navbar/>
        </>
    }
}