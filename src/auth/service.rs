use sqlx::PgPool;

use crate::auth::{
    errors::Error,
    model::{LoginInput, RegisterInput, SlimUser},
    utils::{hash_password, verify_password},
};

pub struct AuthService;

impl AuthService {
    pub async fn log_in(input: LoginInput, pool: &PgPool) -> Result<SlimUser, Error> {
        let user = sqlx::query!(
            r#"select id, username, email, password_hash from users where username = $1"#,
            input.username
        )
        .fetch_one(pool)
        .await?;

        let password_hash = &user.password_hash.ok_or(Error::InvalidUserPassword)?;

            if verify_password(password_hash, &input.password) {
                return Ok(SlimUser {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                });
        }

        Err(Error::AccessDenied)
    }

    pub async fn register(input: RegisterInput, pool: &PgPool) -> Result<SlimUser, Error> {
        let password_hash = hash_password(&input.password);
        let user_id = sqlx::query_scalar(
            r#"
                insert into users (username, email, password_hash)
                values (?, ?, ?)
                returning id
            "#,
        )
        .bind(&input.username)
        .bind(&input.email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(SlimUser {
            id: user_id,
            username: input.username,
            email: input.email,
        })
    }
}
