use crate::{mongo_db::DB};
use crate::{WebResult};
use crate::models::UserRequest;
use warp::{http::StatusCode, reject, reply::json, Reply};



pub async fn list_users(db: DB) ->WebResult<impl Reply> {
    let users = db.fetch_users().await.map_err(|e| reject::custom(e))?;
    Ok(json(&users))
}

pub async fn create_user(body: UserRequest, db: DB) -> WebResult<impl Reply> {
    db.create_user(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}