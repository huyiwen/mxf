use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm_rocket::Connection;

use mxf_entity::errors::JieguoResponse;
use mxf_entity::LeaseData;
use mxf_service::OrderService;

use super::{Claims, OrderDb};

#[post("/lease", data = "<lease_data>")]
async fn lease(
    user: Claims,
    conn: Connection<'_, OrderDb>,
    order_service: &State<OrderService>,
    lease_data: Json<LeaseData>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    Ok(Json(JieguoResponse {
        jieguo: true,
        reason: None,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![lease]
}
