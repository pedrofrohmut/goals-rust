use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use jsonwebtoken::encode;
use serde::{Deserialize, Serialize};

use crate::entities::user::User;

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

pub fn generate_auth_token(user: &User) -> Result<String, String> {
    let now = chrono::Utc::now();
    let one_day_duration = chrono::Duration::hours(24);
    let expiration = (now + one_day_duration).timestamp() as usize;

    let claims = Claims {
        sub: user.get_id().to_string(),
        exp: expiration,
    };

    let secret = "JWT_SECRET"; // Change it to env value on production

    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_ref());
    let token = encode(&header, &claims, &key).map_err(|err| err.to_string())?;

    Ok(token)
}
