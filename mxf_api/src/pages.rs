use rocket::request::FlashMessage;

use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, HouseDb};
use mxf_entity::HouseFilter;
use mxf_service::HouseService;

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
async fn index_alias() -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/zufang?<house_filter..>")]
async fn zufang(
    conn: Connection<'_, HouseDb>,
    house_service: &State<HouseService>,
    house_filter: HouseFilter<'_>,
) -> Result<Template, Flash<Redirect>> {
    let db = conn.into_inner();

    let (houses, num_pages) = house_service
        .find_houses_in_page(db, house_filter, DEFAULT_POSTS_PER_PAGE)
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;

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
async fn user_info(user: Claims) -> Template {
    Template::render("user", context! { user_id: user.name })
}

#[get("/login")]
async fn login_success(_user: Claims) -> Redirect {
    Redirect::to(uri!(user_info))
}

#[get("/login", rank = 2)]
async fn login(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

pub fn routes() -> Vec<Route> {
    routes![index, index_alias, zufang, user_info, login_success, login]
}
