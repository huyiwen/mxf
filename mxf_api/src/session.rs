use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use sea_orm_rocket::Connection;
use serde::Serialize;

use super::claims::Claims;
use mxf_service::UserService;

#[derive(FromForm)]
struct LoginRequest<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
struct LoginResponse<'r> {
    token: &'r str,
}

use super::{claims::JWT_COOKIE_NAME, UserDb};

/// Tries to authenticate a user. Successful authentications get a JWT
#[post("/login", data = "<login>")]
async fn login(
    jar: &CookieJar<'_>,
    conn: Connection<'_, UserDb>,
    user_service: &State<UserService>,
    login: Form<LoginRequest<'_>>,
) -> Result<Redirect, Flash<Redirect>> {
    let db = conn.into_inner();

    user_service
        .login(db, login.username, login.password)
        .await
        .map_err(|e| e.to_redirect(uri!(super::pages::login)))?;

    let token = match Claims::from_name(&login.username).into_token() {
        Ok(token) => token,
        Err(e) => {
            return Err(Flash::error(
                Redirect::to(uri!(super::pages::login)),
                format!("Error creating token: {}", e.1),
            ));
        }
    };
    jar.add_private((JWT_COOKIE_NAME, token));

    Ok(Redirect::to(uri!(super::pages::index)))
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
    routes![login, logout]
}
