use actix_web::{
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    web,
};
use serde::Deserialize;
use crate::utils::{
    is_signed_in,
    get_device_and_ajax,
};
use actix_session::Session;
use sailfish::TemplateOnce;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}

pub async fn index_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        //return news_page(session, req).await;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("is_authenticated"))
    }

    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/auth/auth.stpl")]
            struct DesctopAuthTemplate {
                is_ajax: bool,
            }
            let body = DesctopAuthTemplate {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/auth/auth.stpl")]
            struct MobileAuthTemplate {
                is_ajax: bool,
            }
            let body = MobileAuthTemplate {is_ajax: is_ajax,}
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
