use crate::entities::user::{User, CreateUserDto};
use crate::db::establish_connection;
use crate::data_access::user_data_access::add_user;
use crate::errors::user_errors::InvalidUserError;

mod sign_up {}

pub enum SignUpError {
    DbError(tokio_postgres::Error),
    RequestValidationError(InvalidUserError),
}

pub async fn execute(new_user: CreateUserDto) -> Result<(), SignUpError>
{
    let user = User::from_create_user_dto(new_user).map_err(|err| {
        SignUpError::RequestValidationError(err)
    })?;

    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        SignUpError::DbError(err)
    })?;

    match add_user(&client, &user).await {
        Err(err) => Err(SignUpError::DbError(err)),
        Ok(_) => Ok(()),
    }
}
