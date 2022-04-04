use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    model::User,
    service,
};

lazy_static::lazy_static! {
    pub static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

pub fn sign(id: Uuid) -> Result<String, ApiError> {
    let iat = Utc::now();
    let exp = iat + Duration::hours(24);

    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        },
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| ApiError::AccessDenied)?;
        // Extract postgres pool extension from request
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(|err| ApiError::from(err))?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default()
        )
        .map_err(|err| ApiError::from(err))?;
        // Get the user from the database
        let user = service::find_user(token_data.claims.sub, &pool).await?;

        Ok(user)
    }
}
