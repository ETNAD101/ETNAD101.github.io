use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct BillState {
    pub bill_toggle: u8,
}
