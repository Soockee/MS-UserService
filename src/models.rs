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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginResponse {
    pub jwt_token: String,
}