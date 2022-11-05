use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use lib_authentication::db::Pool;
use lib_authentication::errors::ServiceError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::future::{ready, Ready};
use validator::Validate;

use crate::prelude::*;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = crate::models::User;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(
            min = 1,
            max = 20,
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(
        length(
            min = 1,
            max = 20,
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub bio: Option<String>,
    #[validate(url(message = "is not a URL"))]
    pub image: Option<String>,
}

// JSON response objects ↓
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub token: String,
}

// Route handlers ↓

/// Post new user
pub async fn post_user(
    params: web::Json<RegisterUser>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let register_user = params.into_inner();

    match register_user.validate() {
        Ok(()) => {
            let res = web::block(move || {
                let mut conn = state.get().unwrap();
                crate::db::users::insert_new_user(&mut conn, register_user)
            })
            .await??;

            Ok(HttpResponse::Ok().json(&res))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

/// Login user
pub async fn login(
    req: HttpRequest,
    params: web::Json<LoginUser>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let login_user = params.into_inner();

    match login_user.validate() {
        Ok(()) => {
            let user = web::block(move || {
                let mut conn = state.get().unwrap();
                crate::db::users::verify_user(&mut conn, login_user)
            })
            .await??;

            let token = lib_authentication::auth::create_jwt(user.id, user.username.clone())?;

            Identity::login(&req.extensions(), token.clone()).unwrap();

            let session = Session {
                user_id: user.id,
                email: user.email,
                username: user.username,
                avatar_url: user.image,
                token,
            };

            Ok(HttpResponse::Ok().json(session))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

/// Logout user
pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}
