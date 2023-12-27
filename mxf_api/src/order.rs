use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm_rocket::Connection;

use mxf_entity::errors::JieguoResponse;
use mxf_entity::HnoData;
use mxf_service::{HouseService, OrderService};

use super::{Claims, MXFDb};

#[post("/lease", data = "<lease_data>")]
async fn lease(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    order_service: &State<OrderService>,
    house_service: &State<HouseService>,
    lease_data: Json<HnoData>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let hno = lease_data.hno;
    let db = conn.into_inner();

    let ono = order_service
        .check_available(db, hno)
        .await
        .map_err(|e| e.to_json())?;

    let hlandlore = house_service
        .get_house_by_hno(db, hno)
        .await
        .map_err(|e| e.to_json())?
        .hlandlore;

    println!("Lease: ono = {}, hlandlore = {}", ono, hlandlore);
    order_service
        .place_order_by_ono(db, ono, hno, hlandlore, user.user.uno)
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
    conn: Connection<'_, MXFDb>,
    order_service: &State<OrderService>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    order_service
        .confirm_order(conn.into_inner(), *ono, user.user.uno)
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
