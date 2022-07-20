use actix_web::web::ServiceConfig;

use crate::views::{
    pages,
    auth,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(auth::auth_routes)
    ;
}
