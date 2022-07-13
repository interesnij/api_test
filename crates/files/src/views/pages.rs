use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use serde::Deserialize;

pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}

pub async fn index_page(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("hello, I files server.")
}
