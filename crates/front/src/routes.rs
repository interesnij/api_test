use actix_web::web::ServiceConfig;

use crate::views::{
    pages,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    ;
}
