pub mod models;
pub mod routes;
mod errors;
mod vars;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use actix_cors::Cors;
    use actix_files::Files;
    use crate::routes::routes;
    use actix_redis::RedisSession;
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        let static_files = Files::new("/static", "static/").show_files_listing();
        App::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .service(static_files)
            .configure(routes)
    })
    .bind("194.58.90.123:8100")?
    .run()
    .await
}
