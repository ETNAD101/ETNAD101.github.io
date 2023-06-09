mod components;
mod pages;
mod backend;
use pages::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}