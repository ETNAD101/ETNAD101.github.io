use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::backend::route::Route;
use crate::backend::category::Category;

#[function_component(Navbar)]
pub fn navbar() -> Html {

    let navigator = use_navigator().unwrap();

    let on_change = Callback::from(move |event: Event| {
        let route = event.target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        
        navigator.push(&Route::Menu { category: route });
    });

    html! {
        <div class={"navbar"}>
            <label for="category">{"Menu"}</label>

            <select name="category" onchange={on_change}>
                <option value="Apppetizer">{"Apppetizer"}</option>
                <option value="Drink">{"Drink"}</option>
                <option value="Entree">{"Entree"}</option>
                <option value="Dessert">{"Dessert"}</option>
            </select>
            <a href={"/"}><h1>{"Menu"}</h1></a>
            <a href={"/bill"}><h1>{"Bill"}</h1></a>
        </div>
    }
}