use serde::{Deserialize, Serialize};
use argonautica::{Hasher, Verifier};
use actix_web::{
  http::header::CONTENT_TYPE,
  HttpRequest,
};
use crate::{errors::AuthError, vars, models::SessionUser};
use crate::models::User;


#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}
#[derive(Deserialize, Debug)]
pub struct PhoneJson {
    pub code: String,
}
#[derive(Deserialize)]
pub struct JsonPosition {
    pub key:   i32,
    pub value: i16,
}
#[derive(Serialize, Deserialize)]
pub struct NewListValues {
    pub pk:    i32,
    pub name:  String,
    pub image: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct JsonItemReactions {
    pub data: Vec<i32>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page:    Option<i32>,
    pub is_ajax: Option<bool>,
}

pub fn hash_password(password: &str) -> String {
  Hasher::default()
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .hash()
      .expect("E.")
      //.map_err(|_| AuthError::AuthenticationError(String::from("Не удалось хэшировать пароль")))
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AuthError> {
  Verifier::default()
      .with_hash(hash)
      .with_password(password)
      .with_secret_key(vars::secret_key().as_str())
      .verify()
      .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось подтвердить пароль")))
}

pub fn is_json_request(req: &HttpRequest) -> bool {
    req
      .headers()
      .get(CONTENT_TYPE)
      .map_or(
        false,
        |header| header.to_str().map_or(false, |content_type| "application/json" == content_type)
      )
}

pub fn is_signed_in(session: &Session) -> bool {
  match get_current_user(session) {
      Ok(_) => true,
      _ => false,
  }
}

pub fn set_current_user(session: &Session, user: &SessionUser) -> () {
    // сериализация в строку подходит для этого случая,
    // но двоичный код был бы предпочтительнее в производственных вариантах использования.
    session.insert("user", serde_json::to_string(user).unwrap()).unwrap();
}

pub fn get_current_user(session: &Session) -> Result<SessionUser, AuthError> {
    let msg = "Не удалось извлечь пользователя из сеанса";

    session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from(msg)))
        .unwrap()
        .map_or(
          Err(AuthError::AuthenticationError(String::from(msg))),
          |user| serde_json::from_str(&user).or_else(|_| Err(AuthError::AuthenticationError(String::from(msg))))
        )
}

pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, bool) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = false;
    let mut _type = true;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = true;
        }
    }

    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = false;
            }
        }
    };
    (_type, is_ajax)
}
