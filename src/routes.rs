use warp::{self, Filter};

use crate::repository;
use crate::mongo_db::DB;

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn user_list(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::get())
        .and(with_db(db))
        .and_then(repository::list_users)
}

fn create_user(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("user")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(repository::create_user)
}

fn update_user(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
     warp::path!("user" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(repository::update_user)
}

fn delete_user(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
       .and(warp::delete())
       .and(with_db(db))
       .and_then(repository::delete_user)
}

// fn get_user(db:DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("user" / String)
//         .and(warp::get())
//         .and(with_db(db))
//         .and_then(db::get_user)
// }

// fn update_user(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("user" / String)
//         .and(warp::put())
//         .and(json_body())
//         .and(with_db(db))
//         .and_then(db::update_user)
// }

// fn delete_user(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("user" / String)
//         .and(warp::delete())
//         .and(with_db(db))
//         .and_then(db::delete_user)
// }
pub fn user_routes(db: DB) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    update_user(db.clone())
    .or(delete_user(db.clone()))    
    .or(create_user(db.clone()))
    .or(user_list(db.clone()))

    // get_user(db.clone())
    //     .or(update_user(db.clone()))
    //     .or(delete_user(db.clone()))
    //     .or(create_user(db.clone()))
    //     .or(user_list(db.clone()))
}