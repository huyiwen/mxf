use chrono;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};
use sea_orm_rocket::Connection;

use super::{Claims, MXFDb};
use mxf_entity::user::UserType;
use mxf_entity::{HouseFilter, MXFError, ListStatus};
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
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
    house_filter: HouseFilter<'_>,
) -> Result<Template, Flash<Redirect>> {
    let db = conn.into_inner();
    println!("{}", db.ping().await.is_ok());

    let (houses, num_pages) = house_service
        .get_houses_in_page(db, house_filter, DEFAULT_POSTS_PER_PAGE, true)
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
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    let db = conn.into_inner();
    if hno.is_none() {
        return Err(Flash::error(Redirect::to("/zufang"), "房屋编号不能为空"));
    }

    let house = house_service
        .get_house_by_hno(db, hno.unwrap())
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;
    let orders = order_service
        .get_orders_by_hno(db, hno.unwrap())
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
            hunlisted: house.hunlisted,
            hdate: chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            orders: orders,
            is_admin: user.map(|u| u.user.utype != UserType::User).unwrap_or(false),
        },
    ))
}

#[get("/mine")]
async fn mine(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    match user.user.utype {
        UserType::Admin => {
            let orders = order_service
                .get_orders(conn.into_inner())
                .await
                .map_err(|e| e.to_redirect(uri!(index)))?;
            let orders = OrderService::filter_latest(&orders);
            let shown = vec![true; 8];
            Ok(Template::render(
                "mine",
                context! {
                    title: "所有订单",
                    user: user.user,
                    orders: orders,
                    shown: shown,
                    count: 9,
                },
            ))
        },
        _ => {
            Ok(Template::render(
                "mine",
                context! {
                    title: "所有订单",
                    user: user.user,
                    count: 0,
                },
            ))
        }
    }
}

#[get("/my_orders")]
async fn my_orders(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    println!("user: {:?}", user.user);
    let db = conn.into_inner();
    let my_orders = order_service
        .get_orders_by_htenant(db, user.user.uno)
        .await
        .map_err(|e| e.to_redirect(uri!(index)))?;
    let my_orders = OrderService::filter_latest(&my_orders);
    let shown = vec![false, true, true, false, true, true, true, false, false];
    let count = shown.iter().filter(|&n| *n).count();

    Ok(Template::render(
        "mine",
        context! {
            title: "我的订单",
            user: user.user,
            orders: my_orders,
            shown: shown,
            count: count,
        },
    ))
}


#[get("/received_orders")]
async fn received_orders(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    order_service: &State<OrderService>,
) -> Result<Template, Flash<Redirect>> {
    println!("user: {:?}", user.user);
    let db = conn.into_inner();
    let received_orders = order_service
        .get_orders_by_hlandlore(db, user.user.uno)
        .await
        .map_err(|e| e.to_redirect(uri!(index)))?;
    let received_orders = OrderService::filter_latest(&received_orders);

    let confirm_tags = received_orders
        .iter()
        .map(|o| if !o.is_confirmed() { "" } else { "disabled" })
        .collect::<Vec<&str>>();
    let shown = vec![false, true, true, false, true, true, true, false, true];
    let count = shown.iter().filter(|&n| *n).count();

    Ok(Template::render(
        "mine",
        context! {
            title: "收到的订单",
            user: user.user,
            orders: received_orders,
            shown: shown,
            count: count,
            confirm: confirm_tags,
        },
    ))
}

#[get("/my_listings")]
async fn my_listings(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
) -> Result<Template, Flash<Redirect>> {
    println!("user: {:?}", user.user);
    let db = conn.into_inner();
    let my_listings = house_service
        .get_houses_by_landlore(db, user.user.uno, false)
        .await
        .map_err(|e| e.to_redirect(uri!(index)))?;

    let shown = vec![true; 9];

    Ok(Template::render(
        "my_listings",
        context! {
            title: "收到的申请",
            user: user.user,
            listings: my_listings,
            shown: shown,
            count: 9,
        },
    ))
}

#[get("/my_orders", rank = 2)]
async fn my_orders_need_login() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/received_orders", rank = 2)]
async fn received_orders_need_login() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/my_listings", rank = 2)]
async fn my_listings_need_login() -> Redirect {
    Redirect::to(uri!(login))
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

#[get("/new")]
async fn new_house() -> Result<Template, Flash<Redirect>> {
    Ok(Template::render(
        "modifyhouse",
        context! { modify: false, title: "新建房屋" },
    ))
}

#[get("/modify?<hno>")]
async fn modify_house(
    hno: u32,
    user: Claims,
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
) -> Result<Template, Flash<Redirect>> {
    let house = house_service
        .get_house_by_hno(conn.into_inner(), hno)
        .await
        .map_err(|e| e.to_redirect("/zufang"))?;
    if house.hlandlore != user.user.uno {
        return Err(
            MXFError::NotLandlore(user.user.uno).to_redirect(uri!(index))
        );
    }
    Ok(Template::render(
        "modifyhouse",
        context! {
            modify: true,
            title: "编辑房屋",
            hno: hno,
            hdistrict: house.hdistrict,
            haddr: house.haddr,
            hlo: house.hlo,
            hflr: house.hflr,
            harea: house.harea,
            hequip: house.hsuite,
            hprice: house.hprice,
            hlandlore: house.hlandlore,
            hunlisted: house.hunlisted,
            is_unlisted: house.hunlisted == ListStatus::Unlisted,
        },
    ))
}

#[get("/modify", rank = 2)]
async fn modify_house_no_hno(user: Claims) -> Redirect {
    Redirect::to(uri!(my_listings))
}

#[get("/modify?<hno>", rank = 3)]
async fn modify_house_need_login(hno: Option<u32>) -> Redirect {
    Redirect::to(uri!(login))
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        index_alias,
        zufang,
        detail,
        mine,
        mine_need_login,
        my_orders,
        my_orders_need_login,
        received_orders,
        received_orders_need_login,
        my_listings,
        my_listings_need_login,
        login_success,
        login,
        register,
        new_house,
        modify_house,
        modify_house_no_hno,
        modify_house_need_login,
    ]
}
