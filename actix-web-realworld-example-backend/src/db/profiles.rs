use crate::{
    handlers::profiles::{ProfileResponse, ProfileResponseInner},
    models::{Follower, User},
};
use diesel::prelude::*;
use lib_authentication::errors::ServiceError;
use uuid::Uuid;

pub fn get_profile(
    conn: &mut PgConnection,
    user_id: Uuid,
    username: String,
) -> Result<ProfileResponse, ServiceError> {
    let user: User = {
        use crate::schema::users::dsl::{username as name, users};
        users.filter(name.eq(username)).first(conn)?
    };

    use crate::schema::followers::dsl::{follower_id, followers, user_id as usr_id};

    let following = followers
        .filter(usr_id.eq(user.id))
        .filter(follower_id.eq(user_id))
        .first::<Follower>(conn)
        .optional()?
        .is_some();

    Ok(ProfileResponse {
        profile: ProfileResponseInner {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        },
    })
}
