/*
Creative Name: ____
-Minimum 10  items
-3 categories(Appetizers, Entrees, Drinks), At least 6 options each section
*/

mod components;
mod pages;
mod backend;
use pages::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}