use crate::backend::menu::MenuItem;

pub struct CartItem {
    person: String,
    item: MenuItem,
}

pub static mut CART: Vec<CartItem> = vec![];