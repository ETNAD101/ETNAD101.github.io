mod components;
mod pages;
mod backend;
mod stores;
use pages::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}