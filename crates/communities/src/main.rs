#[macro_use]
extern crate diesel;
#[macro_use(concat_string)]
extern crate concat_string;

pub mod schema;

#[macro_use]
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
    })
    .bind("194.58.90.123:9002")?
    .run()
    .await
}
