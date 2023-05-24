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
    println!("Execute");
    println!("New User: {:?}", new_user);

    let user = match User::from_create_user_dto(new_user) {
        Err(err) => {
            eprintln!("{}", err);
            return Err(SignUpError::RequestValidationError(err));
        }
        Ok(user) => user,
    };

    let client = match establish_connection().await {
        Err(err) => {
            eprintln!("Client connection error: {}", err);
            return Err(SignUpError::DbError(err));
        }
        Ok(x) => x,
    };

    match add_user(&client, &user).await {
        Err(err) => {
            eprintln!("Add users error: {}", err);
            Err(SignUpError::DbError(err))

        }
        Ok(_) => {
            println!("User created");
            Ok(())
        }
    }
}
