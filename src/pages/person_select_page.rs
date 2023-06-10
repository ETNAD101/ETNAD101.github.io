use yew::prelude::*;
use gloo::console::log;

use crate::backend::menu::get_item_from_id;
use crate::components::navbar::Navbar;
use crate::components::text_input::TextInput;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: u16,
}

#[function_component(PersonSelectPage)]
pub fn person_select(Props {item_id}: &Props) -> Html {
    let item = get_item_from_id(item_id.to_owned()).unwrap();

    let onchange = Callback::from(move |person: String| {
        log!(person);
    });

    html! {
        <>
            <Navbar/>
            <TextInput name={"person"} on_change={onchange}/>
        </>
    }
}