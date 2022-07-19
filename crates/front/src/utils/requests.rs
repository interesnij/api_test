use futures::task::LocalSpawn;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;
use dotenv_codegen::dotenv;
use crate::models::user::*;
use crate::error::Error;
use std::collections::HashMap;

const API_ROOT: &str = dotenv!("API_ROOT");

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

async fn request<U, T>(url: String, method: reqwest::Method, body: &U) -> Result<T, Error>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug ,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, format!("/api_users/v1/{}", url))
        .header("Content-Type", "application/json");


    if let Some(token) = get_token(){
        req = req.bearer_auth(token);
    }

    if allow_body{
        req = req.json(&body);
    }

    log::info!("Request: {:?}", req);
    let res_resp = req.send().await;
    log::info!("Response: {:?}", res_resp);

    match res_resp {
        Ok(resp) => {

        match resp.status().is_success() {
            true => {
                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        log::info!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => match resp.status() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = resp.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    },
        Err(err) => {
            Err(0)
        }
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
