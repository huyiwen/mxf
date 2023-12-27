use sea_orm_rocket::Connection;
use rocket::{Route, State};
use rocket::serde::json::Json;

use super::{Claims, MXFDb};

use mxf_entity::errors::{JieguoResponse, MXFError};
use mxf_service::HouseService;
use mxf_entity::HouseListingModel;


#[post("/new", data = "<house_data>")]
async fn new_house(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
    house_data: Json<HouseListingModel>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let db = conn.into_inner();

    let hno = house_service
        .new_house(db, house_data.into_inner(), user.user.uno)
        .await
        .map_err(|e| e.to_json())?;

    Ok(Json(JieguoResponse {
        jieguo: true,
        reason: Some(hno.to_string()),
    }))
}

#[post("/modify", data = "<house_data>")]
async fn modify_house(
    user: Claims,
    conn: Connection<'_, MXFDb>,
    house_service: &State<HouseService>,
    house_data: Json<HouseListingModel>
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let db = conn.into_inner();

    house_service
        .update_house(db, house_data.into_inner(), user.user.uno)
        .await
        .map_err(|e| e.to_json())?;

    Ok(Json(JieguoResponse {
        jieguo: true,
        reason: None,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![new_house, modify_house]
}
