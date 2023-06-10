use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(NotFound)]
pub fn not_found_page() -> Html {
    html! {
        <>
            <Navbar/>
            <p>{"Page Not Found"}</p>
        </>
    }
}