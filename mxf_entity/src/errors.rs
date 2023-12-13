use rocket::Responder;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug, Responder)]
#[response(status = 500)]
pub enum MXFError {
    #[error("database error")]
    DatabaseError {
        msg: String,
        #[response(ignore)]
        source: DbErr,
    },
}
