use axum::{extract::Extension, Json};
use sqlx::postgres::PgPool;

use crate::errors::ApiError;
use crate::model::{LoginInput, RegisterInput, SlimUser};
use crate::service::AuthService;

pub async fn log_in(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SlimUser>, ApiError> {
    let user = AuthService::log_in(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    Ok(Json(user))
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SlimUser>, ApiError> {
    let user = AuthService::register(input, &pool)
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    Ok(Json(user))
}
