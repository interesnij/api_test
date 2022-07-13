use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
};
use serde::Serialize;
use actix_web::web::Json;

pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/test/", web::get().to(test_page));
}

#[derive(Serialize)]
pub struct TestData {
    pub name:        String,
    pub description: String,
}

pub async fn index_page() -> impl Responder {
    HttpResponse::Ok().body("hello, I central API server.")
}
pub async fn test_page(req: HttpRequest) -> Json<TestData> {
    return Json(TestData {
        name:        "Test name",
        description: "Test description",
    });
}
