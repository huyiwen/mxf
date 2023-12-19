use jsonwebtoken;
use pkcs8;
use rocket::http::uri::Reference;
use rocket::response::{Flash, Redirect};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Serialize)]
pub struct JieguoResponse {
    pub jieguo: bool,
    pub reason: Option<String>,
}

impl JieguoResponse {
    pub fn success_json() -> Json<Self> {
        Json(Self {
            jieguo: true,
            reason: None,
        })
    }
}

#[derive(Error, Debug)]
pub enum MXFError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),

    #[error("jwt error")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("failed to create rsa key")]
    RSACreationError(#[from] pkcs8::spki::Error),

    #[error("failed to decode with rsa")]
    RSADecryptionError(#[from] rsa::Error),

    #[error("user not found: {}", .0)]
    UserNotFound(String),

    #[error("invalid user type: {}", .0)]
    InvalidUserType(String),

    #[error("wrong password")]
    WrongPassword(()),

    #[error("user already exists: {}", .0)]
    UserAlreadyExists(String),

    #[error("cache error")]
    CacheError(Option<String>),

    #[error("unknown error")]
    UnknownError(String),
}

impl MXFError {
    pub fn to_redirect<U>(&self, path: U) -> Flash<Redirect>
    where
        U: TryInto<Reference<'static>>,
    {
        println!("error: {}", self.to_string());
        Flash::error(Redirect::to(path), self.to_string())
    }

    pub fn to_json(&self) -> Json<JieguoResponse> {
        println!("error: {}", self.to_string());
        Json(JieguoResponse {
            jieguo: false,
            reason: Some(self.to_string()),
        })
    }
}
