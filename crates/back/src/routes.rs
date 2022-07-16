use actix_web::web;

use crate::views::{
    pages_routes,
    auth_routes,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages_routes)
    .configure(auth_routes)
    ;
}
