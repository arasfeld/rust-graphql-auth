use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
