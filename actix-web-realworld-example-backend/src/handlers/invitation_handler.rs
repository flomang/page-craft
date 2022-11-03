use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::{email_service::send_invitation, models::Invitation};
use lib_authentication::db::Pool;
use lib_authentication::errors::ServiceError;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn post_invitation(
    request: HttpRequest,
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let claims = lib_authentication::auth::unlock_request(&request)?;
    let uid = Uuid::parse_str(&claims.sub).unwrap();
    // run diesel blocking code
    web::block(move || insert_invitation_and_send(uid, invitation_data.into_inner().email, pool))
        .await??;

    Ok(HttpResponse::Ok().finish())
}

// fn create_invitation(
//     eml: String,
//     pool: web::Data<Pool>,
// ) -> Result<(), crate::errors::ServiceError> {
//     let invitation = dbg!(query(eml, pool)?);
//     send_invitation(&invitation)
// }

// /// Diesel query
// fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, crate::errors::ServiceError> {
//     use crate::schema::invitations::dsl::invitations;

//     let mut conn = pool.get().unwrap();

//     let new_invitation = Invitation::from(eml);

//     let inserted_invitation = diesel::insert_into(invitations)
//         .values(&new_invitation)
//         .get_result(&mut conn)?;

//     Ok(inserted_invitation)
// }

fn insert_invitation_and_send(
    sender_id: uuid::Uuid,
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
    let invitation = dbg!(query(sender_id, eml, pool)?);

    // TODO 
    //send_invitation(&invitation)
    Ok(invitation)
}

/// Diesel query
fn query(
    sender_id: uuid::Uuid,
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = Invitation::new(sender_id, eml);
    let mut conn = pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(&mut conn)?;

    Ok(inserted_invitation)
}
