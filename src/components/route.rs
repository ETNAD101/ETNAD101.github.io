use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/description/:item_name")]
    Description{item_name: u16},
    #[at("/")]
    Home,
    #[at("/bill")]
    Bill,
    #[not_found]
    #[at("/404")]
    NotFound,
}