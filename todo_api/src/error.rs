use rocket_okapi::r#gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::ensure_status_code_exists;
use rocket_okapi::Result as OkapiResult;
use thiserror::Error;
use rocket::response::{status, Responder};
use rocket::serde::json::{json, Json};
use rocket::{response, Request,};

#[derive(Debug, Error)]
pub enum Error {
    // TODO: proper network errors
    #[error("Invalid input data")]
    InvalidInput,

    #[error("File not found")]
    FileNotFound,

    #[error("Error inserting type `{type_name}` into table `{table}`")]
    Insertion {
        table: String,
        type_name: String,
    },

    #[error("Authorization error: invalid username or password")]
    Auth,

    #[error(transparent)]
    GeneralDb(#[from] surrealdb::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Responder<'_, 'static> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let status = match self {
            Error::InvalidInput => rocket::http::Status::BadRequest,
            Error::FileNotFound => rocket::http::Status::NotFound,
            _ => rocket::http::Status::InternalServerError,
        };
        status::Custom(status, Json(json!({ "error": self.to_string() }))).respond_to(req)
    }
}

impl OpenApiResponderInner for Error {
    fn responses(_: &mut OpenApiGenerator) -> OkapiResult<Responses> {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 400);
        ensure_status_code_exists(&mut responses, 404);
        ensure_status_code_exists(&mut responses, 500);

        Ok(responses)
    }
}

pub type Result<T> = std::result::Result<T,Error>;