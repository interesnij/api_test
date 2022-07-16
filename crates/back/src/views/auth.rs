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
    establish_connection,
    is_signed_in,
    verify,
    SessionUser,
    LoginUser2,
    get_user_server_ip,
};
use actix_session::Session;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use futures_util::stream::StreamExt as _;
use futures::StreamExt;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    //config.route("/phone_send/{phone}/", web::get().to(phone_send));
    //config.route("/phone_verify/{phone}/{code}/", web::get().to(phone_verify));
    config.route("/signup/", web::get().to(process_signup));
    //config.route("/mob_register/", web::get().to(mobile_signup));
    config.route("/login/", web::post().to(login));
    config.route("/logout/", web::get().to(logout));
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
fn find_user(data: LoginUser2) -> Result<SessionUser, AuthError> {
    use crate::schema::users::dsl::users;

    let _find_user_url = get_user_server_ip() + &"/users/get_user_session/".to_string() + &data.phone +  &"/".to_string();
    let _request = reqwest::get(_find_user_url).await.expect("E.");
    let new_request = _request.text().await.unwrap();
    let user200: GetSessionFields = serde_json::from_str(&new_request).unwrap();
    let user = GetSessionFields {
        id: user200.id,
        phone: user.phone.clone(),
        password: user.password.clone(),
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

    let _connection = establish_connection();
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

#[derive(Debug, Deserialize)]
pub struct UserLoc {
    pub city:      CityLoc,
    pub region:    RegionLoc,
    pub country:   CountryLoc,
}
#[derive(Debug, Deserialize)]
pub struct CityLoc {
    pub name_ru:    String,
    pub name_en:    String,
}
#[derive(Debug, Deserialize)]
pub struct RegionLoc {
    pub name_ru:    String,
    pub name_en:    String,
}
#[derive(Debug, Deserialize)]
pub struct CountryLoc {
    pub name_ru:    String,
    pub name_en:    String,
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

pub async fn process_signup(session: Session, req: HttpRequest) -> impl Responder {
    use crate::utils::{hash_password, set_current_user};
    use chrono::NaiveDate;
    use crate::models::{
        UserLocation, NewUserLocation,
        UserProfile, NewUserProfile,
        IpUser, NewIpUser,
        DesignSetting, NewDesignSetting,
        UserPrivate, NewUserPrivate,
        UserProfileNotification, NewUserProfileNotification,
    };

    let params = web::Query::<NewUserForm>::from_query(&req.query_string());
     // Если пользователь не аноним, то отправляем его на страницу новостей
    if is_signed_in(&session) {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else if params.is_err() {
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")
    }
    else {

    let _connection = establish_connection();
        let params_2 = params.unwrap();
        let mut get_perm = 1;
        let mut ipaddr: String = String::new();

        if let Some(val) = &req.peer_addr() {
            ipaddr = val.ip().to_string();
            if ipaddr.contains(&"91.239.184.81".to_string()) {
                get_perm = 60;
            };
            //println!("{:?}", location200.city.name_ru);
        };

        let mut get_device = "a";
        for header in req.headers().into_iter() {
            if header.0 == "user-agent" {
                let _val = format!("{:?}", header.1);
                if _val.contains("Mobile"){
                    get_device = "b";
                };
            }
        };

        let get_language = "a";
        let mut get_gender = "a";
        if params_2.gender.clone() == "Fem".to_string() {
            get_gender = "b";
        }
        //let count = User::count_users() + 1;
        let count = 1;
        let link = "/id".to_string() + &count.to_string() + &"/".to_string();

        //let _session_user = SessionUser {
        //    id: _new_user.id,
        //    phone: _new_user.phone,
        //};
        //set_current_user(&session, &_session_user);
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok")
    }
}
