/*mod models;
mod postgres_db;
mod error;
mod repository;
mod routes;
mod rabbit;

use postgres_db::DB;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
*/
#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

use rocket_sync_db_pools::{diesel, database};
use std::io;

use rocket::tokio::task::spawn_blocking;


#[get("/login")]
fn login() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn list_users() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn create_user() -> &'static str {
    "Hello, index!"
}

#[put("/<id>", data = "<new_user>")]
fn update_user()-> &'static str {
    "Hello, index!"
}

#[post("/", data = "<new_user>")]
fn delete_user() -> &'static str {
    "Hello, index!"
}

use serde::Deserialize;

#[derive(Deserialize)]
struct User<'r> {
    guid: &'r str,
    username: &'r str,
    email: &'r str,
}

#[database("pg_user_db")]
struct PgDatabase(diesel::PgConnection);

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("../data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgDatabase::fairing())
        .mount("/user", routes![list, create_user, update_user, delete_user])
        .mount("/", routes![login])
}
