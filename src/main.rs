mod mongo_db;
mod models;
mod error;
mod repository;
mod routes;

use mongo_db::DB;
use warp::{Filter, Rejection};
use simple_logger::SimpleLogger;
type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;


#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let db = DB::init().await.unwrap();
    let user_routes = routes::user_routes(db).recover(error::handle_rejection);
    

    warp::serve(user_routes)
        .run(([0, 0, 0, 0], 13331))
        .await;
    
    println!("Server is running...")
}
