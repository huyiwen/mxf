use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};
use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm_rocket::Connection;

use mxf_entity::errors::JieguoResponse;
use mxf_entity::{LoginData, RegisterData};
use mxf_service::UserService;

use super::claims::{Claims, JWT_COOKIE_NAME};
use super::UserDb;

/// Tries to authenticate a user. Successful authentications get a JWT
#[post("/login", format = "json", data = "<login>")]
async fn login(
    jar: &CookieJar<'_>,
    conn: Connection<'_, UserDb>,
    user_service: &State<UserService>,
    login: Json<LoginData<'_>>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let db = conn.into_inner();

    let user = user_service
        .login(db, &login.0)
        .await
        .map_err(|e| e.to_json())?;

    let token = Claims::from_user(&user)
        .into_token()
        .map_err(|e| e.to_json())?;
    jar.add_private((JWT_COOKIE_NAME, token));

    Ok(JieguoResponse::success_json())
}

/// Tries to authenticate a user. Successful authentications get a JWT
#[post("/register", format = "json", data = "<register>")]
async fn register(
    conn: Connection<'_, UserDb>,
    user_service: &State<UserService>,
    register: Json<RegisterData<'_>>,
) -> Result<Json<JieguoResponse>, Json<JieguoResponse>> {
    let db = conn.into_inner();

    user_service
        .register(db, &register.0)
        .await
        .map(|_| JieguoResponse::success_json())
        .map_err(|e| e.to_json())
}

#[post("/logout")]
async fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private(JWT_COOKIE_NAME);
    Flash::success(
        Redirect::to(uri!(super::pages::login)),
        "Successfully logged out.",
    )
}

pub fn routes() -> Vec<Route> {
    routes![login, register, logout]
}
