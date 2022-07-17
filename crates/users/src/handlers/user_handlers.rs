use actix_web::{web, HttpRequest, Responder, HttpResponse, get, HttpResponseBuilder, http::StatusCode};
use actix_web_httpauth::{extractors::bearer::BearerAuth};
use chrono::Utc;
use rbatis::crud::CRUD;
use crate::{
    AppState,
    config::crypto::{unwrap_jwt, verify_jwt},
    models::{UserDetail, User}
};

pub fn user_scope() -> actix_web::Scope{
    web::scope("/user")
        .service(user_detail)
}

#[get("/detail")]
async fn user_detail(_req: HttpRequest, _state: web::Data<AppState>, _token: BearerAuth) -> impl Responder{
    log::info!("User detail: {}", _token.token());

    let claims = verify_jwt(_token.token().to_string(), _state.key.as_ref()).await;
    if let Err(status) = claims{
        return HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).finish();
    }
    let claims = claims.unwrap();

    let user: Result<User, _> = _state.rb.fetch_by_column("id", claims.id).await;

    match user{
        Ok(user_data) => {

            let body = serde_json::to_string(&UserDetail {
                id: user_data.id,
                first_name: user_data.first_name,
                last_name: user_data.last_name,
                types: user_data.types,
                gender: user_data.gender,
                device: user_data.device,
                language: user_data.language,
                perm: user_data.perm,
                link: user_data.link,
                city: user_data.city,
                status: user_data.status,
                image: user_data.b_avatar,
                birthday: user_data.birthday.to_string(),
                last_activity: user_data.last_activity.to_string(),
            }).unwrap();

            HttpResponse::Ok().body(body)
        },
        Err(_) => {
            HttpResponse::Unauthorized().finish()
        },
    }
}
