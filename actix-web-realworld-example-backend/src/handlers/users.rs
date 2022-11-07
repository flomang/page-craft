use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest, HttpResponse};
use lib_authentication::auth::create_jwt;
use lib_authentication::db::Pool;
use lib_authentication::errors::ServiceError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::future::{ready, Ready};
use validator::Validate;

use crate::models::User;
//use crate::prelude::*;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
// pub type LoggedUser = crate::models::User;

// impl FromRequest for LoggedUser {
//     type Error = Error;
//     type Future = Ready<Result<LoggedUser, Error>>;

//     fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
//         if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
//             if let Ok(user_json) = identity.id() {
//                 if let Ok(user) = serde_json::from_str(&user_json) {
//                     return ready(Ok(user));
//                 }
//             }
//         }

//         ready(Err(ServiceError::Unauthorized.into()))
//     }
// }

// wrapper to adhere to realworld spec
#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
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

#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub token: String,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: create_jwt(user.id, user.username.clone()).unwrap(),
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}


// impl UserResponse {
//     fn create_with_auth(auth: Auth) -> Self {
//         UserResponse {
//             user: UserResponseInner {
//                 token: auth.token,
//                 email: auth.user.email,
//                 username: auth.user.username,
//                 bio: auth.user.bio,
//                 image: auth.user.image,
//             },
//         }
//     }
// }


// Route handlers ↓

/// Post new user
pub async fn register_user(
    params: web::Json<In<RegisterUser>>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let register_user = params.into_inner().user;

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
    params: web::Json<In<LoginUser>>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let login_user = params.into_inner().user;

    match login_user.validate() {
        Ok(()) => {
            let user = web::block(move || {
                let mut conn = state.get().unwrap();
                crate::db::users::verify_user(&mut conn, login_user)
            })
            .await??;

            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => Ok(HttpResponse::BadRequest().json(err)),
    }
}

/// Get user
pub async fn get_current(
    request: HttpRequest,
    state: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // must be logged in
    let (claims, token) = lib_authentication::auth::unlock_request(&request)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub).unwrap();

    let mut response = web::block(move || {
        let mut conn = state.get().unwrap();
        crate::db::users::find_user_by_id(&mut conn, user_id)
    })
    .await??;

    response.user.token = token;
    Ok(HttpResponse::Ok().json(response))
}
