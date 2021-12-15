use bcrypt::verify;
use diesel::prelude::*;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Error, Result,
};
use serde::Deserialize;

use crate::database::{get_db_connection, models::User, schema::users, PgPool};

#[derive(Deserialize)]
struct BasicAuthInput {
    username: String,
    password: String,
}

#[handler]
pub fn basic_auth(pool: Data<&PgPool>, req: Json<BasicAuthInput>) -> Result<Json<User>, Error> {
    let conn = get_db_connection(&pool);
    let user = users::table
        .filter(users::username.eq(&req.username))
        .first::<User>(&conn)
        .unwrap();

    let password_hash = user.password_hash.unwrap();
    if verify(&req.password, &password_hash).unwrap() {
        Ok(Json(user))
    } else {
        Err(Error::new(StatusCode::UNAUTHORIZED))
    }
}
