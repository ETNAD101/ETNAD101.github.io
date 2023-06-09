use yew::prelude::*;

use crate::backend::menu_item::get_item_from_id;
use crate::components::order_button::OrderButton;

#[derive(Properties, PartialEq)]
pub struct DescriptionProps {
    pub item_name: u16,
}


#[function_component(Description)]
pub fn description(DescriptionProps { item_name }: &DescriptionProps) -> Html {
    let item = get_item_from_id(item_name.to_owned()).unwrap();

    html! {
        <div class={"description"}>
            <h1>{format!("{} ⋅ ${}", item.name, item.price)}</h1>
            <img src={item.image.clone()}/>
            <p>{item.description}</p>
            <OrderButton/>
        </div>
    }
}