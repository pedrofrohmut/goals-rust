use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub fn hash_password(password: &str) -> Result<String, String> {
    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|err| err.to_string())?;

    // Verify password against PHC string.
    if argon2.verify_password(password, &password_hash).is_err() {
        Err("Error to hash: The hash produced did not match the password provided".to_string())
    } else {
        Ok(password_hash.to_string())
    }
}

pub fn match_password_and_hash(password: &str, password_hash: &str) -> Result<bool, String> {
    let password = password.as_bytes();
    let password_hash = PasswordHash::new(&password_hash).map_err(|err| err.to_string())?;
    let is_match = Argon2::default()
        .verify_password(password, &password_hash)
        .is_ok();
    Ok(is_match)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    // Require. UTC Timestamp Expiration Date
    exp: usize,
}

// Change it to a env value, secure and long secret, for production
const JWT_SECRET: &str = "JWT_SECRET";

fn get_one_day_expiration() -> usize {
    let now = chrono::Utc::now();
    let one_day_duration = chrono::Duration::hours(24);
    (now + one_day_duration).timestamp() as usize
}

pub fn generate_auth_token(user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: get_one_day_expiration(),
        // exp: chrono::Utc::now().timestamp() as usize, // gen a expired token for testing
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let key = EncodingKey::from_secret(JWT_SECRET.as_ref());
    let token = encode(&header, &claims, &key)?;

    Ok(token)
}

fn decode_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let key = DecodingKey::from_secret(JWT_SECRET.as_ref());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS512);
    let decoded = decode::<Claims>(&token, &key, &validation)?;
    Ok(decoded)
}

pub fn validate_and_get_id_from_token(token: &str) -> Result<String, String> {
    let decoded = decode_token(&token).map_err(|err| err.to_string())?;

    let expiration = decoded.claims.exp;
    let now_in_sec = chrono::Utc::now().timestamp() as usize;
    if expiration < now_in_sec {
        return Err("The token is expired".to_string());
    }

    let user_id = decoded.claims.sub;
    Ok(user_id)
}
