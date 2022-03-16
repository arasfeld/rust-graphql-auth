use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::auth::utils::{hash_password, verify_password};
use crate::database::{
    get_db_connection,
    models::{SlimUser, User},
    schema::users,
    Pool,
};
use crate::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub async fn login(
    pool: web::Data<&Pool>,
    input: web::Json<LoginInput>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &get_db_connection(&pool)?;
    let user = users::table
        .filter(users::username.eq(&input.username))
        .first::<User>(conn)
        .unwrap();

    if let Some(password_hash) = &user.password_hash {
        if verify_password(&password_hash, &input.password) {
            let slim_user: SlimUser = user.into();
            return Ok(HttpResponse::Ok().json(&slim_user));
        }
    }

    Err(ServiceError::Unauthorized)
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    pool: web::Data<&Pool>,
    input: web::Json<RegisterInput>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &get_db_connection(&pool)?;
    let password_hash = hash_password(&input.password);
    let user = diesel::insert_into(users::table)
        .values((
            users::email.eq(&input.email),
            users::username.eq(&input.username),
            users::password_hash.eq(password_hash),
        ))
        .get_result::<User>(conn)?;
    let slim_user: SlimUser = user.into();

    Ok(HttpResponse::Ok().json(&slim_user))
}
