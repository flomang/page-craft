use crate::{
    app::{users::UserResponse, AppState},
    utils::auth::authenticate_token,
};
use async_graphql::*;

use super::{
    articles::{ArticleResponse, GetArticle, ArticlesParams, GetArticles, ArticleListResponse},
    profiles::{GetProfile, ProfileResponse},
    Token,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // get the current logged in user by token
    async fn get_current_user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        Ok(UserResponse::create_with_auth(auth))
    }

    // get profile for username
    async fn get_profile<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetProfile { auth, username }).await??;

        Ok(res)
    }

    // get articles
    async fn get_article<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
    ) -> Result<ArticleResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetArticle { auth, slug }).await??;

        Ok(res)
    }

    // get articles
    async fn get_articles<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        filter: ArticlesParams,
    ) -> Result<ArticleListResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token)
            .await
            .map(|auth| Some(auth))
            .unwrap_or(None);

        let res = state.db.send(GetArticles { auth, params: filter }).await??;

        Ok(res)
    }
}
