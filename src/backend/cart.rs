use crate::backend::menu::MenuItem;

pub struct CartItem {
    person: String,
    items: Vec<MenuItem>,
}

pub static mut CART: Vec<CartItem> = vec![];