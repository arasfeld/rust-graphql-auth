use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    Json
};
use sqlx::postgres::PgPool;

use crate::{
    errors::ApiError,
    graphql::AppSchema,
    model::{AuthPayload, RegisterInput, SignInInput, User},
    service,
    utils::jwt::sign,
};

pub async fn sign_in(
    Json(input): Json<SignInInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AuthPayload>, ApiError> {
    let user = service::sign_in(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    let token = sign(user.id)?;
    Ok(Json(AuthPayload { token, user }))
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AuthPayload>, ApiError> {
    let user = service::register(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    let token = sign(user.id)?;
    Ok(Json(AuthPayload { token, user }))
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn graphql(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
    user: Option<User>,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(user)).await.into()
}
