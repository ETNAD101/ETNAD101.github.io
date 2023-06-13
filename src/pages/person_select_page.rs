use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::use_navigator;

use crate::backend::cart::CartItem;
use crate::backend::menu::get_item_from_id;
use crate::components::navbar::Navbar;
use crate::components::text_input::TextInput;
use crate::stores::cart_store::CartStore;
use crate::backend::route::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: u16,
}

#[function_component(PersonSelectPage)]
pub fn person_select(Props {item_id}: &Props) -> Html {
    let (cart, dispatch) = use_store::<CartStore>();

    let navigator = use_navigator().unwrap();

    let item = get_item_from_id(item_id.to_owned()).unwrap();

    let onchange = Callback::from(move |person: String| {
        let cart_item = CartItem {
            person: person.clone(),
            item: item.clone(),
        };
        
        dispatch.reduce_mut(|cart| cart.cart_items.push(cart_item));
        
        if !cart.people.contains(&person) {
            dispatch.reduce_mut(|cart| cart.people.push(person).clone());
        }

        navigator.push(&Route::Home);
    });

    html! {
        <>
            <Navbar/>
            <TextInput name={"person"} on_change={onchange}/>
        </>
    }
}