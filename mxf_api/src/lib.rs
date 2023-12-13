#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;

pub(self) mod claims;
pub(self) mod database;
pub mod pages;
pub mod session;

pub(self) use claims::Claims;
use database::{HouseDb, UserDb};
use mxf_service::{HouseService, UserService};

pub async fn main() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(HouseDb::init())
        .attach(UserDb::init())
        .manage(HouseService::init())
        .manage(UserService::init())
        .mount("/", FileServer::from(relative!("../static")))
        .mount("/", session::routes())
        .mount("/", pages::routes())
        .attach(Template::fairing())
}
