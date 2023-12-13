use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, Db};
use mxf_entity::{HouseFilter, MXFError};
use mxf_service::CachedDb;

const DEFAULT_POSTS_PER_PAGE: u8 = 10u8;

#[get("/")]
async fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render(
        "index",
        context! {
            title: "主页",
            flash: flash.map(FlashMessage::into_inner),
        },
    )
}

#[get("/index")]
async fn redirect_index() -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/zufang?<house_filter..>")]
async fn zufang(
    conn: Connection<'_, Db>,
    query: &State<CachedDb>,
    house_filter: HouseFilter<'_>,
) -> Result<Template, MXFError> {
    let db = conn.into_inner();

    let (houses, num_pages) = query
        .find_houses_in_page(db, house_filter, DEFAULT_POSTS_PER_PAGE)
        .await
        .expect("Failed to query database");
    Ok(Template::render(
        "zufang",
        context! {
            title: "租房",
            flash: ("success", house_filter.to_string()),
            preload: house_filter,
            items: houses,
            max_page: num_pages,
        },
    ))
}

#[get("/user")]
async fn user_page(_user: Claims) -> Template {
    Template::render("user", ())
}

#[get("/login")]
async fn login(_user: Claims) -> Redirect {
    Redirect::to(uri!(user_page))
}

#[get("/login", rank = 2)]
async fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

pub fn routes() -> Vec<Route> {
    routes![index, redirect_index, zufang, user_page, login, login_page]
}
