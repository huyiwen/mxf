use rocket::http::{CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::Route;
use serde::{Deserialize, Serialize};
use rocket::form::Form;
use rocket::response::{Redirect, Flash};

use super::claims::Claims;


#[derive(FromForm)]
struct LoginRequest<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
struct LoginResponse<'r> {
    token: &'r str,
}

use super::claims::JWT_COOKIE_NAME;


/// Tries to authenticate a user. Successful authentications get a JWT
#[post("/login", data = "<login>")]
async fn login(jar: &CookieJar<'_>, login: Form<LoginRequest<'_>>) -> Result<Redirect, Flash<Redirect>> {
    // This should be real user validation code, but is left simple for this example
    if login.username != "username" || login.password != "password" {
        return Err(Flash::error(Redirect::to(uri!(super::pages::login_page)),
            "Invalid username/password."));
    }

    let token = match Claims::from_name(&login.username).into_token() {
        Ok(token) => token,
        Err(e) => {
            return Err(Flash::error(Redirect::to(uri!(super::pages::login_page)),
                format!("Error creating token: {}", e.1)));
        }
    };
    jar.add_private((JWT_COOKIE_NAME, token));

    Ok(Redirect::to(uri!(super::pages::index)))
}

#[post("/logout")]
async fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove_private(JWT_COOKIE_NAME);
    Flash::success(Redirect::to(uri!(super::pages::login_page)), "Successfully logged out.")
}

pub fn routes() -> Vec<Route> {
    routes![login, logout]
}
