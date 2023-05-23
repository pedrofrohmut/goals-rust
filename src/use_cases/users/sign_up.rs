use std::error::Error;

use crate::entities::user::{User, CreateUserDto};
use crate::db::establish_connection;
use crate::data_access::user_data_access::add_user;

mod sign_up {}

pub async fn execute(new_user: CreateUserDto) -> Result<(), Box<dyn Error>>
{
    println!("Execute");
    println!("New User: {:?}", new_user);

    let user = match User::from_create_user_dto(new_user) {
        Err(err) => {
            eprintln!("{}", err);
            return Err(Box::new(err));
        }
        Ok(user) => user,
    };

    let client = match establish_connection().await {
        Err(err) => {
            eprintln!("Client connection error: {}", err);
            return Err(Box::new(err));
        }
        Ok(x) => x,
    };

    match add_user(&client, &user).await {
        Err(err) => {
            eprintln!("Add users error: {}", err);
            Err(Box::new(err))
        }
        Ok(_) => {
            println!("User created");
            Ok(())
        }
    }
}
