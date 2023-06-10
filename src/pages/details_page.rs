use yew::prelude::*;

use crate::backend::menu::get_item_from_id;
use crate::components::order_button::OrderButton;
use crate::components::navbar::Navbar;

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub item_name: u16,
}


#[function_component(DetailsPage)]
pub fn detials(DetailProps { item_name }: &DetailProps) -> Html {
    let item = get_item_from_id(item_name.to_owned()).unwrap();

    html! {
        <>
            <Navbar/>
            
            <div class={"details"}>
                <h1>{format!("{} ⋅ ${}", item.name, item.price)}</h1>
                <img src={item.image.clone()}/>
                <p>{item.description}</p>
                <OrderButton/>
            </div>
        </>
    }
}