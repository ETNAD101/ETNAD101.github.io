use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={"navbar"}>
            <a href={"/"}><h1>{"Menu"}</h1></a>
            <a href={"/bill"}><h1>{"Bill"}</h1></a>
        </div>
    }
}