use bcrypt::verify;
use diesel::prelude::*;
use poem::web::Data;
use poem_openapi::{auth::Basic, ApiResponse, OpenApi, SecurityScheme};

use super::AuthApi;
use crate::database::{get_db_connection, models::User, schema::users, PgPool};

/// Basic authorization
///
/// - User: `test`
/// - Password: `123456`
#[derive(SecurityScheme)]
#[oai(type = "basic")]
struct BasicAuthorization(Basic);

#[derive(ApiResponse)]
enum BasicAuthResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok,
    /// Return when the user credentials do not match.
    #[oai(status = 401)]
    Unauthorized,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[OpenApi]
impl AuthApi {
    #[oai(path = "/basic", method = "get")]
    async fn auth_basic(&self, pool: Data<&PgPool>, auth: BasicAuthorization) -> BasicAuthResponse {
        let conn = get_db_connection(&pool);
        let user = match users::table
            .filter(users::username.eq(auth.0.username))
            .first::<User>(&conn)
        {
            Ok(user) => user,
            Err(_) => return BasicAuthResponse::NotFound,
        };

        match user.password_hash {
            Some(hash) => {
                if verify(auth.0.password, hash.as_ref()).unwrap() {
                    return BasicAuthResponse::Ok;
                } else {
                    return BasicAuthResponse::Unauthorized;
                }
            }
            None => BasicAuthResponse::Unauthorized,
        }
    }
}
