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
mod database_update;
mod rabbit;
mod models;

#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};
use rocket::State;
use std::io;

use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

use rocket::tokio::task::spawn_blocking;

#[get("/login")]
fn login(state: State<CommInterface>) -> &'static str {

}

#[get("/list")]
fn list_users(state: State<CommInterface>) -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn get_user(state: State<CommInterface>, id: usize) -> &'static str {

}


#[post("/<id>", data = "<new_user>")]
fn create_user(state: State<CommInterface>)-> &'static str {
    "Hello, index!"
}

#[put("/<id>", data = "<new_user>")]
fn update_user(state: State<CommInterface>)-> &'static str {
    "Hello, index!"
}

#[delte("/", data = "<id>")]
fn delete_user(state: State<CommInterface>) -> &'static str {
    "Hello, index!"
}

use serde::Deserialize;
use crate::models::CommInterface;
use r2d2_postgres::postgres::NoTls;
use amiquip::Connection;

#[derive(Deserialize)]
struct User<'r> {
    guid: &'r str,
    username: &'r str,
    email: &'r str,
}

#[launch]
fn rocket() -> _ {

    let manager = PostgresConnectionManager::new(get_database_url().parse().unwrap(), NoTls);
    let pool_size: u32 = 1;

    let postgres = Pool::builder().max_size(pool_size).build(manager).unwrap();;
    let rabbit = Connection::insecure_open(&url)?;

    let comm_interface = CommInterface{ postgres, rabbit };

    database_update::DatabaseUpdater::update(&mut comm_interface.postgres.get().unwrap());

    rocket::build()
        .attach(PgDatabase::fairing())
        .mount("/user", routes![list, create_user, update_user, delete_user])
        .mount("/", routes![login])
        .manage(comm_interface)
}
