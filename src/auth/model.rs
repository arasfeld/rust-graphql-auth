use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct SlimUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
