use rocket::{
    http::uri::Reference,
    response::{Flash, Redirect},
};
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MXFError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),

    #[error("creating token error")]
    TokenError(String),

    #[error("user not found")]
    UserNotFound(String),

    #[error("wrong password")]
    WrongPassword(String),

    #[error("cache error")]
    CacheError(String),
}

impl MXFError {
    pub fn to_redirect<U>(&self, path: U) -> Flash<Redirect>
    where
        U: TryInto<Reference<'static>>,
    {
        Flash::error(Redirect::to(path), self.to_string())
    }

    pub fn user_not_found() -> Self {
        MXFError::UserNotFound("user not found".into())
    }

    pub fn wrong_password() -> Self {
        MXFError::WrongPassword("wrong password".into())
    }

    pub fn cache_error() -> Self {
        MXFError::CacheError("cache error".into())
    }
}
