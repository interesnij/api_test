use crate::schema;
use actix_web::{
    HttpRequest,
    web,
    web::Json,
};
use crate::models::{User, GetSessionFields};
use serde::{Serialize, Deserialize};
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::utils::establish_connection;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    config.route("/users/get_user_session/{phone}/", web::get().to(get_user_session));
}

pub async fn get_user_session(phone: web::Path<String>) -> Json<GetSessionFields> {
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let clone_phone = phone.clone();
    let users_by_phone = users
        .filter(schema::users::phone.eq(clone_phone))
        .load::<User>(&_connection)
        .expect("E");

    if users_by_phone.len() > 0 {
        let user = users_by_phone.into_iter().nth(0).unwrap();
        return user.get_session_fields_json()
    }
    else {
        return Json( GetSessionFields {
            id:       0,
            phone:    "".to_string(),
            password: "".to_string(),
        })
    }
}
