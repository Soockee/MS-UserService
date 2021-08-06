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

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, index!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/world/<name>")]
fn hello(name: &str) -> String {
    format!("World name, {}!", name)
}

use serde::Deserialize;


#[derive(Deserialize)]
struct Task<'r> {
    description: &'r str,
    complete: bool
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
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![world])
        .mount("/", routes![blocking_task])
        .mount("/", routes![delay])
}
