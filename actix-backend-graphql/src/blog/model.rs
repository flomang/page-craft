use async_graphql::*;
use validator::Validate;

use crate::{app::{
    users::{LoginUser, RegisterUser, UserResponse, UpdateUser, UpdateUserOuter},
    AppState,
}, utils::auth::{authenticate_token}};

pub struct Token(pub String);
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

pub struct MutationRoot;

#[Object]
impl MutationRoot {

    // register a new user
    async fn signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        email: String,
        password: String,
    ) -> Result<UserResponse> {
        let register_user = RegisterUser {
            username,
            email,
            password,
        };
        register_user.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(register_user).await??;
        Ok(res)
    }

    // login a user
    async fn signin<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        email: String,
        password: String,
    ) -> Result<UserResponse> {
        let login_user = LoginUser { email, password };
        login_user.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(login_user).await??;
        Ok(res)
    }

    // update a user
    async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        bio: Option<String>,
        image: Option<String>,
    ) -> Result<UserResponse> {
        let update_user = UpdateUser { username, email, password, bio, image };
        update_user.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let token =  ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        let res = state
            .db
            .send(UpdateUserOuter { auth, update_user })
            .await??;
        Ok(res)
    }
}
