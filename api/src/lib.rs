pub(self) mod claims;
pub(self) mod house_filter;
pub mod pages;
pub mod session;

pub(self) use claims::Claims;
pub(self) use house_filter::HouseFilter;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;

use service::Db;

#[macro_use]
extern crate rocket;

pub async fn main() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Db::init())
        .mount("/", FileServer::from(relative!("../static")))
        .mount("/", session::routes())
        .mount("/", pages::routes())
        .attach(Template::fairing())
}
