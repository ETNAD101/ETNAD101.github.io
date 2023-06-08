/*
Creative Name: ____
-Minimum 10  items
-3 categories(Appetizers, Entrees, Drinks), At least 6 options each section
*/

mod components;
mod pages;
mod app;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}