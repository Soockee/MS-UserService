use serde::{Deserialize, Serialize};
use amiquip::Connection;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserRequest {
    pub guid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Debug)]
pub struct MessageQueue {
    pub rabbit: Connection
}

#[derive(Queryable)]
pub struct User {
    guid: Uuid,
    username: String,
    email: String,
}
