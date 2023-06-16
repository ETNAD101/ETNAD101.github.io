# comp-sci-final-project

## main.rs
This is the main file that links the whole project together
```Rust
mod components;
mod pages;
mod backend;
mod stores;
mod tests;
use pages::app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}
```