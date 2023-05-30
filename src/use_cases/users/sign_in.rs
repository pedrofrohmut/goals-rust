use tokio_postgres::Client;

use crate::{
    data_access::user_data_access::find_user_by_email,
    db::establish_connection,
    entities::user::{CredentialsDto, SignedUserDto, User},
    services::auth_services::{match_password_and_hash, generate_auth_token},
};

pub enum SignInError {
    InvalidRequestError(String),
    UserNotFound(String),
    PasswordAndHashDontMatchError(String),
    DatabaseError(String),
    GenerateJwtError(String),
}

pub async fn execute(credentials: CredentialsDto) -> Result<SignedUserDto, SignInError> {
    let user = User::from_credentials_dto(credentials)
        .map_err(|err| SignInError::InvalidRequestError(err.to_string()))?;

    let client = get_connected_client().await?;

    let found_user = find_user(&client, &user).await?;

    let password = user.get_password();
    let hash = found_user.get_password_hash();
    check_password(&password, &hash)?;

    let token = make_token(&found_user)?;

    Ok(SignedUserDto {
        id: found_user.get_id().to_string(),
        name: found_user.get_name(),
        email: found_user.get_email(),
        token: token.into(),
    })
}

async fn get_connected_client() -> Result<tokio_postgres::Client, SignInError> {
    // Calls up db util to get a connected client
    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        SignInError::DatabaseError(err.to_string())
    })?;
    Ok(client)
}

async fn find_user(client: &Client, user: &User) -> Result<User, SignInError> {
    let email = user.get_email();

    let found_user = find_user_by_email(&client, &email)
        .await
        .map_err(|err| SignInError::DatabaseError(err.to_string()))?;

    let found_user = found_user.ok_or_else(|| {
        SignInError::UserNotFound(format!("User not found for the e-mail: {}", &email))
    })?;

    Ok(found_user)
}

fn check_password(password: &str, hash: &str) -> Result<(), SignInError> {
    let is_match = match_password_and_hash(&password, &hash)
        .map_err(|err| SignInError::PasswordAndHashDontMatchError(err))?;

    if !is_match {
        return Err(SignInError::PasswordAndHashDontMatchError(
            "Password and Hash don't match".to_string(),
        ));
    }

    Ok(())
}

fn make_token(user: &User) -> Result<String, SignInError>
{
    let user_id = user.get_id();

    let token = generate_auth_token(&user_id).map_err(|err| {
        SignInError::GenerateJwtError(err.to_string())
    })?;

    Ok(token)
}
