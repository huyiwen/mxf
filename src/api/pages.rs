
use rocket::Route;
use rocket::response::Redirect;
use rocket::request::FlashMessage;
use rocket_dyn_templates::{Template, context};

use super::claims::Claims;
use super::queries::HouseFilter;

#[get("/")]
async fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("index", context! {
        title: "主页",
        flash: flash.map(FlashMessage::into_inner),
    })
}

#[get("/zufang?<house_fliter..>")]
async fn zufang(house_fliter: HouseFilter<'_>) -> Template {
    Template::render("zufang", context! {
        title: "租房",
        flash: ("success", house_fliter.to_string()),
        preload: house_fliter,
        items: vec![
            context! {
                hno: "H123456789",
                hdistrict: "北京",
                haddr: "地址",
                hlo: "房型",
                hflr: 1,
                haera: 60,
                hequip: "空调",
                hprice: 1000,
                hlandlore: "U123456789",
                hdate: "2023-10-09 00:10:20"
            },
        ],
    })
}

/// test used to verify that the user is authenticated
#[get("/private")]
async fn private(user: Claims) -> Template {
    Template::render("session", context! {
        user_id: user.name,
    })
}

/// test used to verify that the user is authenticated
#[get("/private", rank = 2)]
async fn no_auth_private() -> Redirect {
    Redirect::to(rocket::uri!(login_page))
}

#[get("/login")]
async fn login(_user: Claims) -> Redirect {
    Redirect::to(uri!(private))
}

#[get("/login", rank = 2)]
async fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

pub fn routes() -> Vec<Route> {
    routes![index, zufang, private, no_auth_private, login, login_page]
}
