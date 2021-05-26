use crate::{mongo_db::DB};
use crate::{WebResult};
use crate::models::{UserRequest, LoginRequest};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn list_users(db: DB) ->WebResult<impl Reply> {
    let users = db.fetch_users().await.map_err(|e| reject::custom(e))?;
    Ok(json(&users))
}

pub async fn create_user(body: UserRequest, db: DB) -> WebResult<impl Reply> {
    db.create_user(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn update_user(id: String, body: UserRequest, db: DB)  -> WebResult<impl Reply> {
    db.update_user(&id, &body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_user(id: String, db: DB)  -> WebResult<impl Reply> {
    db.delete_user(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn login(body: LoginRequest, db: DB) ->WebResult<impl Reply> {
    let user = db.fetch_user_by_username(body.username).await.map_err(|e| reject::custom(e))?;
    Ok(json(&user))
}