use jsonwebtoken::{errors, decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::SystemTime;

const SECRET: &[u8] = b"secret";
pub const EXPIRATION_TIME: u64 = 3600;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    username: String,
    exp: u64,
}

fn now() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub fn token(username: String) -> Result<String, errors::Error> {
    let claims = Claims {
        username: username,
        exp: now() + EXPIRATION_TIME
    };
    let key = EncodingKey::from_secret(SECRET);
    encode(&Header::default(), &claims, &key)
}

pub fn username(token: String) -> Result<String, errors::Error> { 
    let token = token.trim_start_matches("Bearer ");
    let key = DecodingKey::from_secret(SECRET);
    let validation = Validation::default();
    decode::<Claims>(&token, &key, &validation).map(|x| x.claims.username)
}

pub fn cookie(token: String) -> String {
    format!("token={token}; HttpOnly; SameSite=Strict; Max-Age={EXPIRATION_TIME}; Path=/")
}

