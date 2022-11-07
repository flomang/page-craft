use crate::{
    handlers::users::{LoginUser, RegisterUser, UserResponse, UpdateUser},
    models::{NewUser, User, UserChange},
};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;
use lib_authentication::auth::hash_password;
use lib_authentication::errors::ServiceError;

/// Insert new user
pub fn insert_new_user(conn: &mut PgConnection, msg: RegisterUser) -> Result<User, ServiceError> {
    // import user schema
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        username: msg.username.clone(),
        email: msg.email.clone(),
        password: hash_password(&msg.password)?,
        bio: None,
        image: None,
    };

    let user = diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)?;

    Ok(user)
}

/// Login user
pub fn verify_user(conn: &mut PgConnection, msg: LoginUser) -> Result<UserResponse, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let user = users.filter(email.eq(&msg.email)).first::<User>(conn)?;
    if lib_authentication::auth::verify(&user.password, &msg.password)? {
       return Ok(user.into());
    } 

    Err(ServiceError::Unauthorized(json!("invalid password")))
}


/// Find user by user id
pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<UserResponse, ServiceError> {
    use crate::schema::users::dsl::{users, id};

    let user = users.filter(id.eq(user_id)).first::<User>(conn)?;

    Ok(user.into())
}

/// Update user
pub fn update_user(conn: &mut PgConnection, user_id: Uuid, msg: UpdateUser) ->Result<UserResponse, ServiceError> {
    use crate::schema::users::dsl::users;

    let updated_password = match msg.password {
        Some(updated_password) => Some(hash_password(&updated_password)?),
        None => None,
    };

    let updated_user = UserChange {
        username: msg.username,
        email: msg.email,
        password: updated_password,
        bio: msg.bio,
        image: msg.image,
    };

    let user = diesel::update(users.find(user_id))
        .set(&updated_user)
        .get_result::<User>(conn)?;

    Ok(user.into())

} 