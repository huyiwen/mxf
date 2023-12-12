use rocket_dyn_templates::Template;
use rocket::fs::{relative, FileServer};

#[macro_use]
extern crate rocket;

mod api;


#[shuttle_runtime::main]
async fn start() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", api::session::routes())
        .mount("/", api::pages::routes())
        .attach(Template::fairing());

    Ok(rocket.into())
}
