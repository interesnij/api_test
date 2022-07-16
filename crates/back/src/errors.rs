use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::convert::From;

#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    //#[display(fmt = "BadId")]
    //BadId,

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    //#[display(fmt = "ProcessError: {}", _0)]
    //ProcessError(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),
}


impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            //AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),

            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),

            //AuthError::ProcessError(ref message) => HttpResponse::InternalServerError().json(message),

            AuthError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),

            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
