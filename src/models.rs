use serde::{Deserialize, Serialize};
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::NoTls;
use amiquip::Connection;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: String,
    pub guid: String,
    pub username: String,
}

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
pub struct CommInterface {
    pub postgres: Pool<PostgresConnectionManager<NoTls>>,
    pub rabbit: Connection
}