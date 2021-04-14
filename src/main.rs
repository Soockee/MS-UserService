
mod controllers;
mod db;
mod models;
mod routes;

use warp;

#[tokio::main]
async fn main() {
    let db = db::init_db();
    let user_routes = routes::user_routes(db);

    warp::serve(user_routes)
        .run(([127, 0, 0, 1], 13331))
        .await;
}
