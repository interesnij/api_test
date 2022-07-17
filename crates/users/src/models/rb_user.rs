use actix_web::{body::{MessageBody, BoxBody}, Responder, HttpResponse, http::header::ContentType};
use rbatis::{crud_table, DateTimeNative, DateNative};
use serde::{Serialize, Deserialize};

///// Типы пользоватетеля
    // 1 стандартный тип пользователя
    // 3 ребенок
    // 7 идентифицированный
    // 6 пославший запрос на идентификацию
    // 11 удаленный стандартный
    // 13 удаленный ребенок
    // 17 удаленный идентифицированный
    // 16 удаленный пославший запрос на идентификацию
    // 21 закрытый стандартный
    // 23 закрытый ребенок
    // 27 закрытый идентифицированный
    // 26 закрытый пославший запрос на идентификацию
    // 31 приостановленный стандартный
    // 33 приостановленный ребенок
    // 37 приостановленный идентифицированный
    // 36 приостановленный пославший запрос на идентификацию
    // 41 закрытый баннером стандартный
    // 43 закрытый баннером ребенок
    // 47 закрытый баннером идентифицированный
    // 46 закрытый баннером пославший запрос на идентификацию

///// Полномочия пользоватетеля
    // 1 стандартные полномочия
    // 10 TRAINEE_MODERATOR
    // 13 MODERATOR
    // 16 HIGH_MODERATOR
    // 19 TEAMLEAD_MODERATOR
    // 20 TRAINEE_MANAGER
    // 23 MANAGER
    // 26 HIGH_MANAGER
    // 29 TEAMLEAD_MANAGER
    // 30 ADVERTISER
    // 34 HIGH_ADVERTISER
    // 39 TEAMLEAD_ADVERTISER
    // 40 ADMINISTRATOR
    // 44 HIGH_ADMINISTRATOR
    // 49 TEAMLEAD_ADMINISTRATOR
    // 60 SUPERMANAGER

///// Пол пользоватетеля
    // 'a' Мужик
    // 'b' Баба

///// Оборудование пользоватетеля
    // 'a' Комп
    // 'b' Телефон

///// Язык пользоватетеля
    // 'a' Русский
    // 'b' Английский

#[crud_table(table_name: users)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id:            u64,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         u16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          u16,
    pub level:         u16,
    pub password:      String,
    pub link:          String,
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub b_avatar:      Option<String>,
    pub s_avatar:      Option<String>,
    pub email:         Option<String>,
    pub birthday:      chrono::NaiveDate,
    pub last_activity: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    pub id:            u64,
    pub first_name:    String,
    pub last_name:     String,
    pub types:         u16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          u16,
    pub link:          String, // community.get_link()
    pub city:          Option<String>,
    pub status:        Option<String>,
    pub image:         Option<String>,
    pub birthday:      String,
    pub last_activity: String,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct UserToken {
    pub token: String
}

#[derive(Deserialize)]
pub struct UserLogin{
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserSignup {
    pub id:            u64,
    pub first_name:    String,
    pub last_name:     String,
    pub phone:         String,
    pub types:         i16,
    pub gender:        String,
    pub device:        String,
    pub language:      String,
    pub perm:          i16,
    pub level:         i16,
    pub password:      String,
    pub link:          String,
    pub birthday:      DateNative,
    pub last_activity: DateTimeNative,
}


impl Responder for UserDetail {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
