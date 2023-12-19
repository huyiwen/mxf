use chrono;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, HouseDb};
use mxf_entity::HouseFilter;
use mxf_service::{HouseService, UserService};

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

#[get("/detail?<hno>")]
async fn detail(
    conn: Connection<'_, HouseDb>,
    hno: Option<u32>,
    house_service: &State<HouseService>,
) -> Result<Template, Flash<Redirect>> {
    if hno.is_none() {
        return Err(Flash::error(Redirect::to("/zufang"), "房屋编号不能为空"));
    }
    let db = conn.into_inner();
    let house = house_service
        .find_house_by_id(db, hno.unwrap())
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;
    Ok(Template::render(
        "housedetail",
        context! {
            title: "详情",
            hno: hno,
            hdistrict: house.hdistrict,
            haddr: house.haddr,
            hlo: house.hlo,
            hflr: house.hflr,
            harea: house.harea,
            hequip: house.hsuite,
            hprice: house.hprice,
            hlandlore: house.hlandlore,
            hdate: chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        },
    ))
}

#[get("/detail", rank = 2)]
async fn detail_no_hno() -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/admin")]
async fn admin(user: Option<Claims>) -> Template {
    Template::render("adminpage", context! { title: "管理" })
}

#[get("/employee")]
async fn employee(user: Option<Claims>) -> Template {
    Template::render("employeepage", context! { title: "员工" })
}

#[get("/user")]
async fn user_info(user: Claims) -> Template {
    Template::render("user", context! { user_id: user.name })
}

#[get("/user", rank = 2)]
async fn user_info_need_login() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/login")]
async fn login_success(_user: Claims) -> Redirect {
    Redirect::to(uri!(user_info))
}

#[get("/login", rank = 2)]
async fn login(
    flash: Option<FlashMessage<'_>>,
    user_service: &State<UserService>,
) -> Result<Template, Flash<Redirect>> {
    // let public_key = user_service
    //     .get_public_key()
    //     .await
    //     .map_err(|e| e.to_redirect(uri!(register)))?;
    Ok(Template::render(
        "login",
        context! { flash: flash.map(FlashMessage::into_inner) /*, public_key: public_key */ },
    ))
}

#[get("/register")]
async fn register(
    flash: Option<FlashMessage<'_>>,
    user_service: &State<UserService>,
) -> Result<Template, Flash<Redirect>> {
    // let public_key = user_service
    //     .get_public_key()
    //     .await
    //     .map_err(|e| e.to_redirect(uri!(register)))?;
    Ok(Template::render(
        "register",
        context! { flash: flash.map(FlashMessage::into_inner)/*, public_key: public_key */ },
    ))
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        index_alias,
        zufang,
        detail,
        detail_no_hno,
        admin,
        employee,
        user_info,
        user_info_need_login,
        login_success,
        login,
        register,
    ]
}
