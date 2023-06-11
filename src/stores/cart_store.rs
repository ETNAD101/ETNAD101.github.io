use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

use crate::backend::cart::CartItem;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct CartStore {
    pub cart_items: Vec<CartItem>,
}
