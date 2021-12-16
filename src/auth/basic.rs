use actix_web::{web, HttpResponse};
use bcrypt::verify;
use diesel::prelude::*;
use serde::Deserialize;

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

pub async fn basic_auth(
    pool: web::Data<&Pool>,
    input: web::Json<LoginInput>,
) -> Result<HttpResponse, ServiceError> {
    let conn = &get_db_connection(&pool)?;
    let user = users::table
        .filter(users::username.eq(&input.username))
        .first::<User>(conn)
        .unwrap();

    if let Some(password_hash) = &user.password_hash {
        if let Ok(_matching) = verify(&input.password, &password_hash) {
            let slim_user: SlimUser = user.into();
            return Ok(HttpResponse::Ok().json(&slim_user));
        }
    }

    Err(ServiceError::Unauthorized)
}
