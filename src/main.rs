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


pub mod models;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] mod schema;

use diesel::*;

use rocket::tokio::time::{sleep, Duration};
use rocket::State;
use std::io;

use crate::models::{MessageQueue, User};
use rocket::tokio::task::spawn_blocking;
use rocket_sync_db_pools::{database};
use amiquip::Connection;

use self::schema::users::dsl::*;

const NAME:&str = "msCloudBois";
const PASSWORD:&str = "_420this_is_madness_1337";
const HOST:&str = "master.ms.depressive.life:5672";

#[database("pg_user_db")]
struct DbConn(diesel::PgConnection);

/*#[get("/login")]
fn login(state: State<CommInterface>) -> &'static str {

}*/

#[get("/list")]
fn list_users(conn: DbConn) -> &'static str {
    let data = users.load::<User>(conn)?;
    "Hi"
}

/*#[get("/<guid>")]
fn get_user(conn: PgConnection, guid: i64) -> &'static str {
    users.filter(guid.eq(&guid))
}
*/

/*#[post("/<id>", data = "<new_user>")]
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
}*/

#[launch]
fn rocket() -> _ {
/*
    let url:String = format!("amqp://{}:{}@{}", NAME, PASSWORD, HOST);
    let rabbit = Connection::insecure_open(&url)?;

    let comm_interface = MessageQueue { postgres, rabbit };

    database_update::DatabaseUpdater::update(&mut comm_interface.postgres.get().unwrap());
*/
    rocket::build()
        .attach(DbConn::fairing)
        .mount("/user", routes![list_users])
        //.mount("/", routes![login])
        //.manage(comm_interface)
}
