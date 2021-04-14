use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::db::Db;
use crate::models::User;

pub async fn list_user(db: Db) -> Result<impl warp::Reply, Infallible> {
    let users = db.lock().await;
    let users: Vec<User> = users.clone();
    Ok(warp::reply::json(&users))
}

pub async fn create_user(new_user: User, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;
    for user in users.iter() {
        if user.guid == new_user.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }
    users.push(new_user);
    Ok(StatusCode::CREATED)
}

// Why Box: Dynamic Dispatching https://doc.rust-lang.org/1.8.0/book/trait-objects.html
// Reply can be either StatusCode::NOT_FOUND or warp::reply::json because both implement warp::Reply
pub async fn get_user(guid: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let users = db.lock().await;

    for user in users.iter() {
        if user.guid == guid {
            return Ok(Box::new(warp::reply::json(&user)));
        }
    }
    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_user(
    guid: String,
    updated_user: User,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;
    for user in users.iter_mut() {
        if user.guid == guid {
            *user = updated_user;
            return Ok(StatusCode::OK);
        }
    }
    return Ok(StatusCode::NOT_FOUND);
}

pub async fn delete_user(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut users = db.lock().await;
    let user_count = users.len();

    users.retain(|user| user.guid != guid);
    let deleted = users.len() != user_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
