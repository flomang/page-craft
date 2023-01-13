use async_graphql::*;
use validator::Validate;

use crate::{
    app::{
        users::{LoginUser, RegisterUser, UpdateUser, UpdateUserOuter, UserResponse},
        AppState,
    },
    utils::auth::authenticate_token,
};

use super::{
    profiles::{FollowProfile, ProfileResponse, UnfollowProfile},
    Token, articles::{ArticleResponse, CreateArticle, CreateArticleOuter},
};
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
        let update_user = UpdateUser {
            username,
            email,
            password,
            bio,
            image,
        };
        update_user.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        let res = state
            .db
            .send(UpdateUserOuter { auth, update_user })
            .await??;
        Ok(res)
    }

    // follow a user
    async fn follow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;

        let res = state.db.send(FollowProfile { auth, username }).await??;

        Ok(res)
    }

    // unfollow a user
    async fn unfollow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;

        let res = state.db.send(UnfollowProfile { auth, username }).await??;

        Ok(res)
    }

    // create article
    async fn create_acticle<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
    ) -> Result<ArticleResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;

        let article = CreateArticle {
            title,
            description,
            body,
            tag_list,
        };
        article.validate()?;

        let res = state
            .db
            .send(CreateArticleOuter { auth, article })
            .await??;

        Ok(res)
    }
}
