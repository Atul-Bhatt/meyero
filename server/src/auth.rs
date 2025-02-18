use actix_web::{http::header, FromRequest, HttpRequest, dev};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub token: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iat: i64, // issued at
    pub exp: i64, // expiry
    pub email: String,
    pub role: String,
}

lazy_static::lazy_static! {
    static ref BEARER_REGEXP : Regex = Regex::new(r"^Bearer\s(.*)$").expect("Bearer regex failed!");
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|authorization| {
                BEARER_REGEXP
                    .captures(authorization)
                    .and_then(|captures| captures.get(1))
            })
            .map(|v| v.as_str());

        futures::future::ready(Ok(match token {
            None => AuthUser {token: None},
            Some(token) => match decode_token(token) {
                Ok(decoded) => AuthUser {token: decoded},
                Err(_) => AuthUser {token: None},
            },
        }))
    }
}