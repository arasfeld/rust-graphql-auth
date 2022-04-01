use axum::{extract::Extension, Json};
use sqlx::postgres::PgPool;

use crate::auth::{
    errors::Error,
    model::{LoginInput, RegisterInput, SlimUser},
    service::AuthService,
};

pub async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SlimUser>, Error> {
    let user = AuthService::log_in(input, &pool)
        .await
        .map_err(|_| Error::AccessDenied)?;
    Ok(Json(user))
}

pub async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SlimUser>, Error> {
    let user = AuthService::register(input, &pool)
        .await
        .map_err(|_| Error::AccessDenied)?;
    Ok(Json(user))
}
