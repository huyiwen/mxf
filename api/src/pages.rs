use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::Route;
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, HouseFilter};
use service::{Db, Query};

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

#[get("/zufang?<page>&<house_filter..>")]
async fn zufang(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    house_filter: Option<HouseFilter<'_>>,
) -> Template {
    let house_filter = house_filter.unwrap_or_default();
    let db = conn.into_inner();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    if page == 0 {
        panic!("Page number cannot be zero");
    }
    let (houses, num_pages) = Query::find_houses_in_page(db, page, DEFAULT_POSTS_PER_PAGE)
        .await
        .expect("Cannot find houses in page");
    Template::render(
        "zufang",
        context! {
            title: "租房",
            flash: ("success", house_filter.to_string()),
            preload: house_filter,
            items: houses,
            maxPage: num_pages,
        },
    )
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
