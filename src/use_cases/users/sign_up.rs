use crate::entities::user::{User, CreateUserDto};
use crate::db::establish_connection;
use crate::data_access::user_data_access::add_user;
use crate::errors::user_errors::InvalidUserError;
use crate::services::auth_services::{hash_password, match_password_and_hash};

mod sign_up {}

pub enum SignUpError {
    RequestValidationError(InvalidUserError),
    HashPasswordError(String),
    DbError(tokio_postgres::Error),
}

pub async fn execute(new_user: CreateUserDto) -> Result<(), SignUpError>
{
    let mut user = User::from_create_user_dto(new_user).map_err(|err| {
        SignUpError::RequestValidationError(err)
    })?;

    let password_hash = hash_password(&user.get_password()).map_err(|err| {
        SignUpError::HashPasswordError(err)
    })?;
    let is_match = match_password_and_hash(&user.get_password(), &password_hash).map_err(|err| {
        SignUpError::HashPasswordError(err)
    })?;
    if ! is_match {
        return Err(SignUpError::HashPasswordError("Hash is not a match".to_string()));
    }
    user.set_password_hash(password_hash);


    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        SignUpError::DbError(err)
    })?;

    match add_user(&client, &user).await {
        Err(err) => Err(SignUpError::DbError(err)),
        Ok(_) => Ok(()),
    }
}
