use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={"navbar"}>
            <label for="category">{"Menu"}</label>

            <select name="category">
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