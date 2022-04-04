use axum::{extract::Extension, Json};
use sqlx::postgres::PgPool;

use crate::{
    errors::ApiError,
    model::AuthPayload,
    service,
    utils::jwt::sign,
};

pub async fn sign_in(
    Json(input): Json<service::SignInInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AuthPayload>, ApiError> {
    let user = service::sign_in(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    let token = sign(user.id)?;
    Ok(Json(AuthPayload {
        token: token,
        user: user,
    }))
}

pub async fn register(
    Json(input): Json<service::RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AuthPayload>, ApiError> {
    let user = service::register(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    let token = sign(user.id)?;
    Ok(Json(AuthPayload {
        token: token,
        user: user,
    }))
}
