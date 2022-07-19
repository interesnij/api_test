use futures::task::LocalSpawn;
use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;


use crate::models::user::*;

struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}


pub fn get_token()-> Option<String>{
    let token = LocalStorage::get("Test-token");
    match token{
        Ok(tok) => Some(tok),
        Err(err) => {
            log::info!("Token doesn't exits");
            None
        },
    }
}

pub fn set_token(token: String){
    LocalStorage::set("Test-token", token).unwrap();
}

pub fn remove_token(){
    LocalStorage::delete("Test-token");
}


pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

pub async fn request_delete<T>(url: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::DELETE, &()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
{
    request(url, reqwest::Method::GET, &()).await
}

/// Post request with a body
pub async fn request_post<U, T>(url: String, body: &U) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::POST, body).await
}

/// Put request with a body
pub async fn request_put<U, T>(url: String, body: &U) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
{
    request(url, reqwest::Method::PUT, body).await
}
