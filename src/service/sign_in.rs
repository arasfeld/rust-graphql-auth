use sqlx::PgPool;

use crate::errors::ApiError;
use crate::model::{SignInInput, User};
use crate::utils::encryption::verify_password;

pub async fn sign_in(input: SignInInput, pool: &PgPool) -> Result<User, ApiError> {
    let user = sqlx::query!(
        r#"select id, username, email, password_hash from users where username = $1"#,
        input.username
    )
    .fetch_one(pool)
    .await?;

    let password_hash = &user.password_hash.ok_or(ApiError::AccessDenied)?;

    if verify_password(password_hash, &input.password) {
        return Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
        });
    }

    Err(ApiError::AccessDenied)
}
