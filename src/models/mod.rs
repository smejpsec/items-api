use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Item {
    pub name: String,
    pub amount: i32,
}

#[derive(Debug, Deserialize)]
pub struct ItemQuery {
    pub name: String,
}
