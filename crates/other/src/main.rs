#[macro_use]
extern crate diesel;
//#[macro_use(concat_string)]
//extern crate concat_string;

pub mod schema;
pub mod models;
pub mod routes;

#[macro_use]
mod utils;
#[macro_use]
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use crate::routes::routes;

    HttpServer::new(|| {
        App::new()
        .configure(routes)
    })
    .bind("194.58.90.123:9004")?
    .run()
    .await
}
