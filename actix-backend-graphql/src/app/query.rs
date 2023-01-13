use async_graphql::*;
use validator::Validate;

use crate::{app::{
    users::{LoginUser, RegisterUser, UserResponse, UpdateUser, UpdateUserOuter},
    AppState,
}, utils::auth::{authenticate_token}};

use super::Token;

pub struct QueryRoot;

#[Object]
impl QueryRoot {

    // get the current logged in user by token
    async fn get_current_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token =  ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        Ok(UserResponse::create_with_auth(auth))
    }
}