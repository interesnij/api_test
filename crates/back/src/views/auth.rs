use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::AuthError;
use serde::{Deserialize, Serialize};
use crate::utils::{
    is_signed_in,
    verify,
    SessionUser,
    LoginUser2,
    get_user_server_ip,
};
use actix_session::Session;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use futures_util::stream::StreamExt as _;
use futures::StreamExt;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    //config.route("/phone_send/{phone}/", web::get().to(phone_send));
    //config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    //config.route("/signup/", web::get().to(process_signup));
    //config.route("/mob_register/", web::get().to(mobile_signup));
    config.route("/login/", web::post().to(login));
    //config.route("/logout/", web::get().to(logout));
}


pub async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().body("ok")
}

#[derive(Deserialize, Serialize)]
pub struct GetSessionFields {
    pub id:       i32,
    pub phone:    String,
    pub password: String,
}
async fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    let _find_user_url = get_user_server_ip() + &"/users/get_user_session/".to_string() + &data.phone +  &"/".to_string();
    let _request = reqwest::get(_find_user_url).await.expect("E.");
    let new_request = _request.text().await.unwrap();
    let user200: GetSessionFields = serde_json::from_str(&new_request).unwrap();
    let user = GetSessionFields {
        id: user200.id,
        phone: user200.phone.clone(),
        password: user200.password.clone(),
    };

    if user.id != 0 {
        if let Ok(matching) = verify(&user.password, &data.password) {
            if matching {
                let __user = SessionUser {
                    id: user.id,
                    phone: user.phone,
                };
                return Ok(__user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in(data: LoginUser2,
                session: &Session,
                req: &HttpRequest) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
    }
}

pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        phone: "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "phone" {
                    form.phone = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, session: Session, req: HttpRequest) -> impl Responder {
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        println!("{:?}", form.phone.clone());
        println!("{:?}", form.password.clone());
        handle_sign_in(form, &session, &req)
    }
}
