//#[macro_use(concat_string)]
//extern crate concat_string;

pub mod routes;

#[macro_use]
mod views;
#[macro_use]
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use actix_redis::RedisSession;
    use crate::routes::routes;

    HttpServer::new(|| {
        App::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .configure(routes)
    })
    .bind("194.58.90.123:8000")?
    .run()
    .await
}
