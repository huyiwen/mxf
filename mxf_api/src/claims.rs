use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use mxf_entity::{MXFError, UserModel};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

/// Key used for symmetric token encoding
const SECRET: &str = "secret";
pub(super) const JWT_COOKIE_NAME: &str = "jwt";

lazy_static! {
    /// Time before token expires (aka exp claim)
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(5);
}

// Used when decoding a token to `Claims`
#[derive(Debug, PartialEq)]
pub(crate) enum AuthenticationError {
    Decoding(String),
    Expired,
}

// Basic claim object. Only the `exp` claim (field) is required. Consult the `jsonwebtoken` documentation for other claims that can be validated.
// The `name` is a custom claim for this API
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Claims {
    pub(crate) name: String,
    pub(crate) user: UserModel,
    exp: usize,
}

// Rocket specific request guard implementation
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthenticationError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(JWT_COOKIE_NAME) {
            None => Outcome::Forward(Status::Unauthorized),
            Some(cookie) => match Claims::from_authorization(cookie.value()) {
                Err(_e) => Outcome::Forward(Status::Forbidden),
                Ok(claims) => Outcome::Success(claims),
            },
        }
    }
}

impl Claims {
    pub(crate) fn from_user(user: &UserModel) -> Self {
        Self {
            name: user.uname.clone(),
            user: user.clone(),
            exp: 0,
        }
    }

    /// Create a `Claims` from a 'Bearer <token>' value
    fn from_authorization(value: &str) -> Result<Self, AuthenticationError> {
        let token = value.trim();

        // Use `jsonwebtoken` to get the claims from a JWT
        // Consult the `jsonwebtoken` documentation for using other algorithms and validations (the default validation just checks the expiration claim)
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthenticationError::Expired,
            _ => AuthenticationError::Decoding(e.to_string()),
        })?;

        Ok(token.claims)
    }

    /// Converts this claims into a token string
    pub(crate) fn into_token(mut self) -> Result<String, MXFError> {
        let expiration = Utc::now()
            .checked_add_signed(*TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();

        self.exp = expiration as usize;

        // Construct and return JWT using `jsonwebtoken`
        // Consult the `jsonwebtoken` documentation for using other algorithms and asymmetric keys
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )?;

        Ok(token)
    }
}
