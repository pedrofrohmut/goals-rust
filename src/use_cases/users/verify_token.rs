use crate::services::auth_services::get_id_from_token;

pub enum VerifyTokenError {
    DecodeTokenError(String),
}

pub fn execute(token: String) -> Result<(), VerifyTokenError> {
    let user_id = get_id_from_token(&token).map_err(|err| {
        VerifyTokenError::DecodeTokenError(err.to_string())
    })?;

    println!("UserId: {}", &user_id);

    Ok(())
}
