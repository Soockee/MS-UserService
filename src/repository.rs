use crate::{mongo_db::DB};
use crate::{WebResult};
use crate::models::*;
//use crate::error::Error::*;
use crate::error::LoginError;
use std::env;
use serde_json::{json};
use reqwest::Client;
use warp::{http::StatusCode, reject, reply::json, Reply};

use log::{info, trace, warn};
use simple_logger::SimpleLogger;

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
    let login_response: LoginResponse;
    let auth_service_base_url;
    let auth_service_endpoint;
    SimpleLogger::new().init().unwrap();
    match env::var("AUTH_SERVICE_URL") {
        Ok(val) => auth_service_base_url = val,
        Err(_) => return Err(reject::custom(LoginError)),
    }
    match env::var("AUTH_SERVICE_LOGIN_ENDPOINT") {
        Ok(val) => auth_service_endpoint = val,
        Err(_) => return Err(reject::custom(LoginError)),
    }
    log::warn!("BaseURL: {}",auth_service_base_url);
    log::warn!("Endpoint: {}",auth_service_endpoint);

    let request_url_test = reqwest::Url::parse(&auth_service_base_url).unwrap();
    let request_url_test = request_url_test.join(&auth_service_endpoint).unwrap();
    log::warn!("Parsed URL: {}", request_url_test.into_string());
    match db.fetch_user_by_username(&body.username).await{
        Ok(_) =>{
            let _body_s = body.clone();
                // TODO: Change Hardcoded URL
                let request_url = reqwest::Url::parse(&auth_service_base_url).unwrap();
                let request_url = request_url.join(&auth_service_endpoint).unwrap();
                let val = json!(&body);
                let client = Client::new();
                let response = client.post(request_url)
                                    .json(&val)
                                    .send();
                match response.await {
                    Ok(success_response) => match success_response.json().await{
                        Ok(success_json) => {
                            login_response = success_json;
                        }
                        Err(_) => {
                            return Err(reject::custom(LoginError))
                        },
                    }
                    Err(_) => {
                            return Err(reject::custom(LoginError))
                    },
                }
        }
        Err(_) =>{
                return Err(reject::custom(LoginError))
        }
    }
    Ok(json(&login_response))
}
