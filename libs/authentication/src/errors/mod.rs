use actix_web::{error::BlockingError, error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde_json::{json, Value as JsonValue};
use std::convert::From;
use uuid::Error as ParseError;

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(JsonValue),

    // 401
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    //#[fail(display = "Unprocessable Entity: {}", _0)]
    //UnprocessableEntity(JsonValue),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            ServiceError::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            //ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<BlockingError> for ServiceError {
    fn from(_: BlockingError) -> ServiceError {
        ServiceError::BadRequest("encountered blocking error".into())
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(json!({ "error": message }));
                }
                ServiceError::InternalServerError
            }
            DieselError::NotFound => {
                ServiceError::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
