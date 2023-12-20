use chrono;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, HouseDb, OrderDb};
use mxf_entity::user::UserType;
use mxf_entity::HouseFilter;
use mxf_service::{HouseService, OrderService, UserService};

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
    hno: Option<u32>,
    user: Option<Claims>,
    house_conn: Connection<'_, HouseDb>,
    house_service: &State<HouseService>,
    order_conn: Connection<'_, OrderDb>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    if hno.is_none() {
        return Err(Flash::error(Redirect::to("/zufang"), "房屋编号不能为空"));
    }

    let house = house_service
        .get_house_by_hno(house_conn.into_inner(), hno.unwrap())
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;
    let orders = order_service
        .get_orders_by_hno(order_conn.into_inner(), hno.unwrap())
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;
    let orders = OrderService::filter_latest(&orders);
    println!("house: {:?} -> orders: {:?}", house, orders);
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
            orders: orders,
            is_admin: user.map(|u| u.user.utype != UserType::User).unwrap_or(false),
        },
    ))
}

#[get("/mine")]
async fn mine(
    user: Claims,
    order_conn: Connection<'_, OrderDb>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    println!("user: {:?}", user.user);
    if user.user.utype != UserType::User {
        let admin_orders = order_service
            .get_orders(order_conn.into_inner())
            .await
            .map_err(|e| e.to_redirect(uri!(index)))?;
        return Ok(Template::render(
            "mine",
            context! {
                is_admin: true,
                user_name: user.name,
                user_id: user.user.uno,
                uphone: user.user.uphone,
                uemail: user.user.uemail,
                my_orders: admin_orders,
            },
        ));
    }

    let order_db = order_conn.into_inner();
    let my_orders = order_service
        .get_orders_by_htenant(order_db, user.user.uno)
        .await
        .map_err(|e| e.to_redirect(uri!(index)))?;
    let my_orders = OrderService::filter_latest(&my_orders);
    let received_orders = order_service
        .get_orders_by_hlandlore(order_db, user.user.uno)
        .await
        .map_err(|e| e.to_redirect(uri!(index)))?;
    let received_orders = OrderService::filter_latest(&received_orders);

    let confirm_tags = received_orders
        .iter()
        .map(|o| if !o.is_confirmed() { "" } else { "disabled" })
        .collect::<Vec<&str>>();

    Ok(Template::render(
        "mine",
        context! {
            is_admin: user.user.utype != UserType::User,
            user_name: user.name,
            user_id: user.user.uno,
            uphone: user.user.uphone,
            uemail: user.user.uemail,
            my_orders: my_orders,
            received_orders: received_orders,
            confirm: confirm_tags,
        },
    ))
}

#[get("/mine", rank = 2)]
async fn mine_need_login() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/login")]
async fn login_success(_user: Claims) -> Redirect {
    Redirect::to(uri!(mine))
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
        mine,
        mine_need_login,
        login_success,
        login,
        register,
    ]
}
