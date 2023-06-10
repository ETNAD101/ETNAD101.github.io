use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::menu_page::MenuPage;
use crate::pages::details_page::DetailsPage;
use crate::pages::bill_page::BillPage;
use crate::pages::not_found_page::NotFound;
use crate::backend::route::Route;

fn switch(routes: Route) -> Html{
    match routes {
        Route::Home => html! {<MenuPage/>},
        Route::Description{item_name} => html! {<DetailsPage item_name={item_name}/>},
        Route::Bill => html! {<BillPage/>},
        Route::NotFound => html!{<NotFound/>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
    <BrowserRouter>
        <Switch<Route> render={switch}/>
    </BrowserRouter>
    }
}