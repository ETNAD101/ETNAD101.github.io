use yew::prelude::*;
use yewdux::prelude::*;
use crate::backend::route::Route;
use yew_router::prelude::use_navigator;
use crate::stores::cart_store::CartStore;

use crate::backend::menu::get_item_from_id;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub person: String,
    pub on_click: Callback<String>
}


#[function_component(PersonButton)]
pub fn person_button(Props {person, on_click}: &Props) -> Html {
    let (cart, dispatch) = use_store::<CartStore>();

        let navigator = use_navigator().unwrap();
    
        let item = get_item_from_id(item_id.to_owned()).unwrap();
    
        let buttons: Html = cart.clone().people.iter().map(|person: &String| {
            let onclick = Callback::from(|_| {
                let cart_item = CartItem {
                    person: person.clone(),
                    item: item,
                };
            });
    
            html! {
                <button class={"personButton"} onclick={onclick}>{person}</button>
            }
        }).collect();
}