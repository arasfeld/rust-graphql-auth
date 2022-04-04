use sqlx::PgPool;

use crate::errors::ApiError;
use crate::model::{RegisterInput, User};
use crate::utils::encryption::hash_password;

pub async fn register(input: RegisterInput, pool: &PgPool) -> Result<User, ApiError> {
    let password_hash = hash_password(&input.password);
    let user_id = sqlx::query_scalar(
        r#"
            insert into users (username, email, password_hash)
            values ($1, $2, $3)
            returning id
        "#,
    )
    .bind(&input.username)
    .bind(&input.email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: user_id,
        username: input.username,
        email: input.email,
    })
}
