//#[macro_use(concat_string)]
//extern crate concat_string;

pub mod routes;
mod errors;
mod vars;

#[macro_use]
mod views;
#[macro_use]
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use crate::routes::routes;

    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
    .bind("194.58.90.123:8000")?
    .run()
    .await
}
