use crate::{
    handlers::users::{LoginUser, RegisterUser, UserResponse},
    models::{NewUser, User},
};
use diesel::prelude::*;
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

    if let Ok(matching) = lib_authentication::auth::verify(&user.password, &msg.password) {
        if matching {
            return Ok(user.into());
        }
    }

    Err(ServiceError::Unauthorized)
}
