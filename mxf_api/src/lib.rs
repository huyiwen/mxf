#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;

pub(self) mod claims;
pub(self) mod database;
pub mod order;
pub mod pages;
pub mod session;
pub mod house_listing;

pub(self) use claims::Claims;
use database::MXFDb;
use mxf_service::{HouseService, OrderService, UserService};


pub async fn main() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(MXFDb::init())
        .manage(HouseService::init())
        .manage(UserService::init())
        .manage(OrderService::init())
        .mount("/", FileServer::from(relative!("../static")))
        .mount("/", pages::routes())
        .mount("/", session::routes())
        .mount("/", order::routes())
        .mount("/", house_listing::routes())
        .attach(Template::fairing())
}
