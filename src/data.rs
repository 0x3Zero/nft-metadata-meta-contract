use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct DataStructFork {
    pub owner: String,
    pub cid: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSeaAttributes {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: i32,
}
