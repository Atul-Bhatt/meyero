use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use serde_json::json;
use crate::AppState;

fn generate_jwt_token(user_id Uuid) -> String {
    let current_time = chrono::Utc::now().timestamp();
    let expiration_time = current_time + 86400; // one day
    let claims = json!({
        "sub": "public_key",
        "exp": expiration_time
    });

    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey.from_secret("jwt_secret".as_ref())).unwrap();
}
