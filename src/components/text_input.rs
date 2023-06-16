use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(Props { name, on_change }: &Props) -> Html {
    let handle_onchange = on_change.clone();
    
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(value);
    });

    html! {
        <input class="textInput" type="text" name={name.clone()} onchange={onchange} />
    }
}