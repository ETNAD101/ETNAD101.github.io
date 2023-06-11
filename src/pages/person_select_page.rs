use yew::prelude::*;
use yewdux::prelude::*;

use crate::backend::cart::CartItem;
use crate::backend::menu::get_item_from_id;
use crate::components::navbar::Navbar;
use crate::components::text_input::TextInput;
use crate::stores::cart_store::CartStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: u16,
}

#[function_component(PersonSelectPage)]
pub fn person_select(Props {item_id}: &Props) -> Html {
    let dispatch = Dispatch::<CartStore>::new();

    let item = get_item_from_id(item_id.to_owned()).unwrap();

    let onchange = Callback::from(move |person: String| {
        let cart_item = CartItem {
            person: person,
            item: item.clone(),
        };
        dispatch.reduce_mut(|cart| cart.cart_items.push(cart_item));
    });

    html! {
        <>
            <Navbar/>
            <TextInput name={"person"} on_change={onchange}/>
        </>
    }
}