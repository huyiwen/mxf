pub(self) mod claims;
pub mod pages;
pub mod session;

pub(self) use claims::Claims;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;

use mxf_service::{pool::SeaOrmPool, CachedDb};

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[macro_use]
extern crate rocket;

pub async fn main() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Db::init())
        .manage(CachedDb::init())
        .mount("/", FileServer::from(relative!("../static")))
        .mount("/", session::routes())
        .mount("/", pages::routes())
        .attach(Template::fairing())
}
