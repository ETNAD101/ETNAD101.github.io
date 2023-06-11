use crate::backend::menu::MenuItem;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CartItem {
    pub person: String,
    pub item: MenuItem,
}
