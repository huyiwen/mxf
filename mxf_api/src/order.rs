use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm_rocket::Connection;

use mxf_entity::errors::JieguoResponse;
use mxf_entity::LeaseData;
use mxf_service::{HouseService, OrderService};

use super::{Claims, HouseDb, OrderDb};

#[post("/lease", data = "<lease_data>")]
async fn lease(
    user: Claims,
    order_conn: Connection<'_, OrderDb>,
    order_service: &State<OrderService>,
    house_conn: Connection<'_, HouseDb>,
    house_service: &State<HouseService>,
    lease_data: Json<LeaseData>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let hno = lease_data.hno;
    let order_db = order_conn.into_inner();

    let ono = order_service
        .check_available(order_db, hno)
        .await
        .map_err(|e| e.to_json())?;

    let hlandlore = house_service
        .get_house_by_hno(house_conn.into_inner(), hno)
        .await
        .map_err(|e| e.to_json())?
        .hlandlore;

    order_service
        .place_order_by_ono(order_db, ono, hno, hlandlore, user.user.uno)
        .await
        .map_err(|e| e.to_json())?;

    Ok(Json(JieguoResponse {
        jieguo: true,
        reason: None,
    }))
}

#[post("/confirm", format = "json", data = "<ono>")]
async fn confirm(
    ono: Json<u32>,
    user: Claims,
    order_conn: Connection<'_, OrderDb>,
    order_service: &State<OrderService>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    order_service
        .confirm_order(order_conn.into_inner(), *ono, user.user.uno)
        .await
        .map_err(|e| e.to_json())?;

    Ok(Json(JieguoResponse {
        jieguo: true,
        reason: None,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![lease, confirm]
}
