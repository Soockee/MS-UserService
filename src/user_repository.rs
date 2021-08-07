use crate::models::{UserRequest, LoginRequest, User};
use r2d2_postgres::r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::{NoTls, Row};
use r2d2_postgres::postgres::error::Error;

pub async fn list_users(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<User<>>, Error> {
    let statement = connection.prepare("SELECT * FROM user")?;
    let mut result: Vec<User> = vec![];
    for row in connection.query(&statement, &[])? {
        result.push(build_user_from_row(&row));
    }
    Ok(result)
}

pub async fn create_user(body: User, connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> User {
    let statement = connection.prepare("INSERT INTO user (guid, username, email) VALUES ($1, $2, $3)")?;
    let rows = connection.query(&statement, &[&body.guid, &body.username, &body.email])?;
    build_user_from_row(&rows[0])
}

pub async fn get_user(guid: i64, connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> User {
    let statement = connection.prepare("SELECT * FROM WHERE guid = $1")?;
    let rows = connection.query(&statement, &[&guid])?;
    build_user_from_row(&rows[0])
}

pub async fn update_user(user: User, connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> User {
    let statement = connection.prepare("UPDATE USER SET (guid, username, email) VALUES ($1, $2, $3) WHERE guid = $1")?;
    let rows = connection.query(&statement, &[&user.guid, &user.username, &user.email])?;
    build_user_from_row(&rows[0])
}

pub async fn delete_user(guid: i64, connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> i64 {
    let statement = connection.prepare("DELETE FROM user WHERE guid = $1")?;
    connection.query(&statement, &[&guid])?;
    guid
}

fn build_user_from_row(row: &Row) -> User{
    let guid: i64 = row.get(0);
    let username: &str = row.get(1);
    let email: &str = row.get(2);
    User{guid, username, email}
}
