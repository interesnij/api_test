use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    is_signed_in,
    verify,
    get_ajax,
    get_user_server_ip,
};
use crate::models::SessionUser;
use actix_session::Session;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use futures_util::stream::StreamExt as _;
use sailfish::TemplateOnce;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/login/", web::post().to(login));
    config.route("/logout/", web::get().to(logout));
}

pub async fn mobile_signup(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/main/auth/signup.stpl")]
        struct NobileSignupTemplate {
            is_ajax: bool,
        }

        let is_ajax = get_ajax(&req);
        let body = NobileSignupTemplate {
            is_ajax: is_ajax,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().body("ok")
}

fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    let url = format!("/users/find_user/{}/{}/", data.phone, data.password);
    let _find_user_url = get_api_server_ip() + &url;
    let _request = reqwest::get(_find_user_url).await.expect("E.");
    let new_request = _request.text().await.unwrap();
    let user: SessionUser = serde_json::from_str(&new_request).unwrap();

    if let Some(user) {
        return Ok(user.into());
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in (
    data: LoginUser2,
    session: &Session,
    req: &HttpRequest
) -> Result<HttpResponse, AuthError> {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub phone:    String,
    pub password: String,
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

#[derive(Deserialize)]
pub struct NewUserForm {
    pub first_name:  String,
    pub last_name:   String,
    pub gender:      String,
    pub password:    String,
    pub birthday:    String,
    pub phone:       String,
}

pub async fn phone_send(_phone: web::Path<String>) -> impl Responder {
    use crate::utils::PhoneJson;
    let req_phone = _phone.to_string();
    if req_phone.len() > 8 {
        use crate::models::{PhoneCode, NewPhoneCode};
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        let _some_user = users
            .filter(schema::users::phone.eq(&req_phone))
            .load::<User>(&_connection)
            .expect("E");
        if _some_user.len() > 0 {
            let rendered = "Пользователь с таким номером уже зарегистрирован. Используйте другой номер или напишите в службу поддержки, если этот номер Вы не использовали ранее.";
            HttpResponse::Ok().body(rendered)
        } else {

            let _url = "https://api.ucaller.ru/v1.0/initCall?service_id=12203&key=GhfrKn0XKAmA1oVnyEzOnMI5uBnFN4ck&phone=".to_owned() + &req_phone;
            let __request = reqwest::get(_url).await.expect("E.");
            let new_request = __request.text().await.unwrap();
            println!("{:?}", new_request);

            let phone200: PhoneJson = serde_json::from_str(&new_request).unwrap();
            let code_i32: i32 = phone200.code.parse().unwrap();
            let new_phone_code = NewPhoneCode {
                phone: _phone.to_string(),
                code:  code_i32,
            };
            diesel::insert_into(schema::phone_codes::table)
                .values(&new_phone_code)
                .get_result::<PhoneCode>(&_connection)
                .expect("E.");

            let rendered = "Мы Вам звоним. Последние 4 цифры нашего номера - код подтверждения, который нужно ввести в поле 'Последние 4 цифры' и нажать 'Подтвердить' <div class='row block_verify mt-5'><div class='col-md-2'></div><div class='col-md-4'><input type='number' id='code' onkeyup='code_check();' class='form-control border-0' placeholder='Последние 4 цифры'><hr class='my-0'></div><div class='mb-3 col-md-4'><button type='button' disabled='disabled' id='code_send' class='btn btn-primary pink-gradient'>Подтвердить</button></div><div class='col-md-2'></div></div>";
            HttpResponse::Ok().body(rendered)
        }
    }
    else {
        let rendered = "Введите, пожалуйста, корректное количество цифр Вашего телефона";
        HttpResponse::Ok().body(rendered)
    }
}

pub async fn phone_verify(param: web::Path<(String,i32)>) -> impl Responder {

    HttpResponse::Ok().body("phone_verify")
}
