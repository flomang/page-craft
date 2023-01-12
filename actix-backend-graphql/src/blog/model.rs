use actix_web::{HttpResponse, HttpRequest};
use async_graphql::*;
use validator::Validate;

use crate::{app::{
    users::{LoginUser, RegisterUser, UserResponse, UpdateUser, UpdateUserOuter},
    AppState,
}, utils::auth::{authenticate2}};

pub struct Token(pub String);
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
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
        let auth = authenticate2(state, token).await?;
        let res = state
            .db
            .send(UpdateUserOuter { auth, update_user })
            .await??;
        Ok(res)
    }
}
