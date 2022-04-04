use async_graphql::{Context, EmptySubscription, Object, Result, Schema};
use sqlx::PgPool;

use crate::{
    errors::ApiError,
    model::{AuthPayload, RegisterInput, SignInInput, User},
    service,
    utils::jwt
};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        Ok(ctx.data::<Option<User>>()?.to_owned())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<AuthPayload> {
        let pool = ctx.data::<PgPool>().map(|pool| pool.to_owned())?;
        let user = service::sign_in(input, &pool)
            .await
            .map_err(|_| ApiError::AccessDenied)?;
        let token = jwt::sign(user.id)?;
        Ok(AuthPayload { token, user })
    }

    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let pool = ctx.data::<PgPool>().map(|pool| pool.to_owned())?;
        let user = service::register(input, &pool).await?;
        let token = jwt::sign(user.id)?;
        Ok(AuthPayload { token, user })
    }
}
