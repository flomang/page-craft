use actix_web::HttpResponse;
use async_graphql::*;
use validator::Validate;

use crate::app::{users::RegisterUser, AppState};

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
    ) -> Result<String> {
        let register_user = RegisterUser {
            username,
            email,
            password,
        };
        register_user.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(register_user).await??;
        Ok(serde_json::to_string(&res).unwrap())
    }

    async fn signin(&self, username: String, _password: String) -> Result<String> {
        Ok(username)
    }
}
