#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;
use shuttle_secrets::SecretStore;
use std::env;

pub(self) mod claims;
pub(self) mod database;
pub mod order;
pub mod pages;
pub mod session;
pub mod house_listing;

pub(self) use claims::Claims;
use database::MXFDb;
use mxf_service::{HouseService, OrderService, UserService};


pub async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> rocket::Rocket<rocket::Build> {
    let env_url = env::var("MYSQL").unwrap_or(String::from("?"));
    let url = secret_store.get("MYSQL").unwrap_or(env_url);
    println!("SQL: {}", url);
    let figment = rocket::Config::figment().merge((
            "databases.mxf",
            sea_orm_rocket::Config {
                url: url,
                min_connections: Some(4),
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: Some(120),
                sqlx_logging: true,
            },
        ));

    rocket::custom(figment)
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
