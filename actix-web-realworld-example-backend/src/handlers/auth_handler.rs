use actix_identity::Identity;
use actix_web::HttpMessage;
use actix_web::{dev::Payload, post, web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use std::future::{ready, Ready};

use crate::models::{SlimUser, UpdateUserPassword, User, Invitation};
use lib_authentication::errors::ServiceError;
use lib_authentication::auth::hash_password;
use lib_authentication::db::Pool;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

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

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}

pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || query(auth_data.into_inner(), pool)).await??;
    let token = lib_authentication::auth::create_jwt(user.id, user.username.clone())?;

    Identity::login(&req.extensions(), token.clone()).unwrap();

    let session = Session {
        user_id: user.id,
        email: user.email,
        username: user.username,
        avatar_url: user.avatar_url,
        token,
    };

    Ok(HttpResponse::Ok().json(session))
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let mut conn = pool.get().unwrap();

    let mut people = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(&mut conn)?;

    if let Some(user) = people.pop() {
        let master = std::env::var("MASTER_EMAIL").expect("MASTER_EMAIL must be set");
        // set auth password if not set for master
        if user.email == master && user.hash == "" {
            let now = Utc::now().naive_utc();
            let password = lib_authentication::auth::hash_password(&auth_data.password)?;
            let set_pwd = UpdateUserPassword {
                id: user.id,
                hash: password,
                updated_at: now,
            };

            let result = diesel::update(users).set(&set_pwd).get_result::<User>(&mut conn);

            match result {
                Ok(u) => return Ok(u.into()),
                Err(e) => return Err(ServiceError::BadRequest(e.to_string())),
            }
        } else if let Ok(matching) = lib_authentication::auth::verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

#[post("/{invitation_id}")]
pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        query_invitation(
            invitation_id.into_inner(),
            user_data.into_inner().password,
            pool,
        )
    })
    .await??;
    Ok(HttpResponse::Ok().json(&res))
}

fn query_invitation(
    invitation_id: String,
    password: String,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};
    use crate::schema::users::dsl::users;
    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

    let mut conn = pool.get().unwrap();
    invitations
        .filter(id.eq(invitation_id))
        .load::<Invitation>(&mut conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // if invitation is not expired
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // try hashing the password, else return the error that will be converted to ServiceError
                    let password: String = hash_password(&password)?;
                    dbg!(&password);
                    let user = User::from_details(invitation.recipient_email, password);
                    let inserted_user: User =
                        diesel::insert_into(users).values(&user).get_result(&mut conn)?;
                    dbg!(&inserted_user);
                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}