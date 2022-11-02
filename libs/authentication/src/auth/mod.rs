use crate::errors::ServiceError;
use actix_web::{dev::ServiceRequest, web, Error, HttpRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use argonautica::{Hasher, Verifier};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod middleware;

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}
//static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds
const ONE_DAY: i64 = 60 * 60 * 24; // in seconds

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    // the subject will be the user-id
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub username: String,
}

pub fn unlock_request(request: &HttpRequest) -> Result<Claims, ServiceError> {
    let authen_header = match request.headers().get("Authorization") {
        Some(authen_header) => authen_header,
        None => {
            return Err(ServiceError::BadRequest(
                "no Authorization header".to_string(),
            ));
        }
    };

    match authen_header.to_str() {
        Ok(authen_str) => {
            if !authen_str.starts_with("bearer") && !authen_str.starts_with("Bearer") {
                return Err(ServiceError::Unauthorized);
            }

            let raw_token = authen_str[6..authen_str.len()].trim();
            let claims = validate_token(&raw_token.to_string(), SECRET_KEY.as_bytes())?;
            Ok(claims)
        }
        Err(err) => {
            log::error!("{}", err);
            return Err(ServiceError::InternalServerError);
        }
    }
}

// Note: bearer_auth_validator returns Error instead of ServiceError
// this is intentional to conform to HttpAuthentication::bearer sig.
pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    if let Ok(_) = validate_token(credentials.token(), &key.as_bytes()) {
        Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}

pub fn decode_token(
    token: String,
    secret: &[u8],
) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<Claims>,
    _pool: &web::Data<crate::db::Pool>,
) -> Result<String, String> {
    Ok(token_data.claims.username.to_string())
    //if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
    //    Ok(token_data.claims.user.to_string())
    //} else {
    //    Err("Invalid token".to_string())
    //}
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, ServiceError> {
    let validation = Validation::new(Algorithm::HS256);

    let data =
        jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
            .map_err(|err| {
                dbg!(err);
                ServiceError::Unauthorized
            })?;

    Ok(data.claims)
}

pub fn create_jwt(user_id: Uuid, username: String) -> Result<String, ServiceError> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + ONE_DAY,
        username,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}


// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}
