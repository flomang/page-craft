use super::users::Auth;
use actix_web::{web, web::Path, HttpRequest, HttpResponse};
use lib_authentication::db::Pool;
use serde::{Deserialize, Serialize};

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct ProfilePath {
    username: String,
}

// Client Messages ↓

#[derive(Debug)]
pub struct GetProfile {
    // auth is option in case authentication fails or isn't present
    pub auth: Option<Auth>,
    pub username: String,
}

#[derive(Debug)]
pub struct FollowProfile {
    pub auth: Auth,
    pub username: String,
}

#[derive(Debug)]
pub struct UnfollowProfile {
    pub auth: Auth,
    pub username: String,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponseInner {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

// Route handlers ↓

pub async fn get_profile(
    request: HttpRequest,
    path: Path<ProfilePath>,
    state: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let profile_path = path.into_inner();
    let (claims, _) = lib_authentication::auth::unlock_request(&request)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub).unwrap();

    let response = web::block(move || {
        let mut conn = state.get().unwrap();
        crate::db::profiles::get_profile(&mut conn, user_id, profile_path.username)
    })
    .await??;

    Ok(HttpResponse::Ok().json(response))

    // ) -> impl Future<Item = HttpResponse, Error = Error> {
    //     let db = state.db.clone();

    //     authenticate(&state, &req)
    //         .then(move |auth| {
    //             db.send(GetProfile {
    //                 auth: auth.ok(),
    //                 username: path.username.to_owned(),
    //             })
    //             .from_err()
    //         })
    //         .and_then(|res| match res {
    //             Ok(res) => Ok(HttpResponse::Ok().json(res)),
    //             Err(e) => Ok(e.error_response()),
    //         })
}

// pub fn follow(
//     state: Data<AppState>,
//     (path, req): (Path<ProfilePath>, HttpRequest),
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let db = state.db.clone();

//     authenticate(&state, &req)
//         .and_then(move |auth| {
//             db.send(FollowProfile {
//                 auth,
//                 username: path.username.to_owned(),
//             })
//             .from_err()
//         })
//         .and_then(|res| match res {
//             Ok(res) => Ok(HttpResponse::Ok().json(res)),
//             Err(e) => Ok(e.error_response()),
//         })
// }

// pub fn unfollow(
//     state: Data<AppState>,
//     (path, req): (Path<ProfilePath>, HttpRequest),
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let db = state.db.clone();

//     authenticate(&state, &req)
//         .and_then(move |auth| {
//             db.send(UnfollowProfile {
//                 auth,
//                 username: path.username.to_owned(),
//             })
//             .from_err()
//         })
//         .and_then(|res| match res {
//             Ok(res) => Ok(HttpResponse::Ok().json(res)),
//             Err(e) => Ok(e.error_response()),
//         })
// }
