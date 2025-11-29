use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    pub base_url: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum MoveAction {
    Hit,
    Stand,
}
