use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub guid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserRequest {
    pub username: String,
}