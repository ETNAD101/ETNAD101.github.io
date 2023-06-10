use yew::prelude::*;

use crate::backend::menu::MenuItem;
use crate::components::order_button::OrderButton;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<MenuItem>,
}

#[function_component(ItemList)]
pub fn itemlist(Props { items }: &Props) -> Html {

    items.iter().map(|item: &MenuItem| {
        html!{
        <div class={"item"}>
            <p>{format!("{} ⋅ ${}", item.name, item.price)}</p>
            <a href={format!("/description/{}", item.id)}><img class={"itemImage"} src={item.image.clone()}/></a>
            <br/>
            <OrderButton item={item.clone()}/>
        </div>
        }
    }).collect()
}
