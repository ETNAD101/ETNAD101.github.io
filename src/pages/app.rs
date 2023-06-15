use yew::prelude::*;
use yew_router::prelude::*;

use crate::backend::category::Category;
use crate::pages::menu_page::MenuPage;
use crate::pages::details_page::DetailsPage;
use crate::pages::bill_page::BillPage;
use crate::pages::not_found_page::NotFound;
use crate::pages::person_select_page::PersonSelectPage;
use crate::backend::route::Route;

fn switch(routes: Route) -> Html{
    match routes {
        Route::Root => html!{<Redirect<Route> to={Route::Menu{category: Category::Appetizer.to_string()}}/>},
        Route::Menu{category} => html! {<MenuPage category={category}/>},
        Route::Description{item_id} => html! {<DetailsPage item_id={item_id}/>},
        Route::Select{item_id} => html! {<PersonSelectPage item_id={item_id}/>},
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