use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::menu::Menu;
use crate::pages::description::Description;
use crate::components::route::Route;



fn switch(routes: Route) -> Html{
    match routes {
        Route::Home => html! {<Menu/>},
        Route::Description{item_name} => html! {<Description item_name={item_name}/>},
        Route::Bill => html! {<p>{"BILL"}</p>},
        Route::NotFound => html!{<p>{"ERROR"}</p>},
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