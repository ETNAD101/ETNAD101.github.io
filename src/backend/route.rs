use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/description/:item_id")]
    Description{item_id: u16},
    #[at("/")]
    Home,
    #[at("/person-select/:item_id")]
    Select{item_id: u16},
    #[at("/bill")]
    Bill,
    #[not_found]
    #[at("/404")]
    NotFound,
}