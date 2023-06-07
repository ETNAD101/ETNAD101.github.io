mod components;

use components::app::App;

/*
Creative Name: ____
-Minimum 10  items
-3 categories(Appetizers, Entrees, Drinks), At least 6 options each section
*/


fn main() {
    yew::Renderer::<App>::new().render();
}