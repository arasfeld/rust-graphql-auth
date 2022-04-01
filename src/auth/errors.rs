use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Access denied")]
    AccessDenied,

    #[error("Invalid username and/or password")]
    InvalidUserPassword,

    #[error("Username taken")]
    UsernameTaken,

    #[error("An error occurred with the database")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::AccessDenied => StatusCode::UNAUTHORIZED,
            Self::InvalidUserPassword => StatusCode::UNAUTHORIZED,
            Self::UsernameTaken => StatusCode::CONFLICT,
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
