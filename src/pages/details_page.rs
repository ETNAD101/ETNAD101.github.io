use yew::prelude::*;

use crate::backend::menu::get_item_from_id;
use crate::components::order_button::OrderButton;
use crate::components::navbar::Navbar;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_id: u16,
}


#[function_component(DetailsPage)]
pub fn detials(Props { item_id }: &Props) -> Html {
    let item = get_item_from_id(item_id.to_owned()).unwrap();

    html! {
        <>
            <Navbar/>
            
            <div class={"details"}>
                <h1>{format!("{} ⋅ ${}", item.name, item.price)}</h1>
                <img src={item.image.clone()}/>
                <p>{item.description.clone()}</p>
                <OrderButton item={item}/>
            </div>
        </>
    }
}