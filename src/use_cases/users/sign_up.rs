use crate::entities::user::{User, CreateUserDto};
use crate::db::establish_connection;
use crate::data_access::user_data_access::{add_user, find_user_by_email, UserDataAccessError};
use crate::errors::user_errors::InvalidUserError;
use crate::services::auth_services::{hash_password, match_password_and_hash};

pub enum SignUpError {
    RequestValidationError(InvalidUserError),
    HashPasswordError(String),
    DbError(UserDataAccessError),
    EmailAlreadyTakenError(String),
}

pub async fn execute(new_user: CreateUserDto) -> Result<(), SignUpError>
{
    let mut user = dto_to_entity_user(new_user)?;

    let password_hash = creates_and_validates_password_hash(&user)?;
    user.set_password_hash(password_hash).map_err(|err| SignUpError::HashPasswordError(err.to_string()))?;

    let client = get_connected_client().await?;

    is_email_available(&client, &user).await?;

    add_user(&client, &user).await.map_err(|err| SignUpError::DbError(err))?;

    Ok(())
}

fn dto_to_entity_user(new_user: CreateUserDto) -> Result<User, SignUpError>
{
    // Creates a valid User entity from CreateUserDto
    let user = User::from_create_user_dto(new_user).map_err(|err| {
        SignUpError::RequestValidationError(err)
    })?;
    Ok(user)
}

fn creates_and_validates_password_hash(user: &User) -> Result<String, SignUpError>
{
    // Creates a PasswordHash and checks if it matches the password
    let password_hash = hash_password(&user.get_password()).map_err(|err| {
        SignUpError::HashPasswordError(err)
    })?;
    let is_match = match_password_and_hash(&user.get_password(), &password_hash).map_err(|err| {
        SignUpError::HashPasswordError(err)
    })?;
    if ! is_match {
        return Err(SignUpError::HashPasswordError("Hash is not a match".to_string()));
    }
    Ok(password_hash)
}

async fn get_connected_client() -> Result<tokio_postgres::Client, SignUpError>
{
    // Calls up db util to get a connected client
    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        SignUpError::DbError(UserDataAccessError::DbError(err))
    })?;
    Ok(client)
}

async fn is_email_available(client: &tokio_postgres::Client, user: &User) -> Result<(), SignUpError>
{
    // Check if e-mail already in use
    let email = user.get_email();
    let found_user = find_user_by_email(&client, &email).await.map_err(|err| {
        SignUpError::DbError(err)
    })?;
    if let Some(_) = found_user {
        return Err(SignUpError::EmailAlreadyTakenError(
                   "E-mail already taken and cannot be used".to_string()));
    }
    Ok(())
}
