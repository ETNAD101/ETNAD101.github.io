use yew::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::text_input::TextInput;

#[function_component(PersonSelectPage)]
pub fn person_select() -> Html {
    html! {
        <>
            <Navbar/>
            <TextInput name={"person"}/>
        </>
    }
}