use warp::{self, Filter};

use crate::controllers;
use crate::db::Db;
use crate::models::User;

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn user_list(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::get())
        .and(with_db(db))
        .and_then(controllers::list_user)
}
fn create_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(controllers::create_user)
}

fn get_user(db:Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(controllers::get_user)
}

fn update_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(controllers::update_user)
}

fn delete_user(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(controllers::delete_user)
}
pub fn user_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_user(db.clone())
        .or(update_user(db.clone()))
        .or(delete_user(db.clone()))
        .or(create_user(db.clone()))
        .or(user_list(db.clone()))
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}