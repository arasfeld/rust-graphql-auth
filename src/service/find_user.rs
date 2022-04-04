use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::model::User;

pub async fn find_user(id: Uuid, pool: &PgPool) -> Result<User, ApiError> {
    let user = sqlx::query!(
        r#"select id, username, email from users where id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        id: user.id,
        username: user.username,
        email: user.email,
    })
}
