use diesel::prelude::*;
use crate::schema;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use serde::Deserialize;
use crate::utils::{
    establish_connection,
};

pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
}

pub async fn index_page(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("hello, I users server.")
}
