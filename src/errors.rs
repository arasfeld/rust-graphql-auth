use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::UnableToConnectToDb => {
                HttpResponse::InternalServerError().json("Unable to connect to DB")
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
