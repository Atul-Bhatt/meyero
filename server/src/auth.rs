use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, Validation};
use actix_web::{http::header, FromRequest, HttpRequest, dev};
use serde::{Deserialize, Serialize};
use regex::Regex;
use serde_json::json;
use uuid::Uuid;

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


pub fn generate_jwt_token(user_id: Uuid) -> String {
    let current_time = chrono::Utc::now().timestamp();
    let expiration_time = current_time + 86400; // one day
    let claims = json!({
        "user_id": user_id,
        "sub": "public_key",
        "exp": expiration_time
    });

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret("jwt_secret".as_ref())).unwrap()
}

pub fn decode_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("jwt_secret".as_ref()).unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| Error(e))
}