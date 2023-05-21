use crate::routes::user_routes::CreateUserBody;

use crate::db::establish_connection;
use crate::data_access::user_data_access::add_user;

mod sign_up {
    pub fn validate()
    {
        println!("Validate");
    }
}

pub async fn execute(new_user: CreateUserBody) -> Result<(), tokio_postgres::Error>
{
    println!("Execute");
    println!("New User: {:?}", new_user);

    sign_up::validate();

    let client = match establish_connection().await {
        Err(err) => {
            eprintln!("Client connection error: {}", err);
            // return HttpResponse::InternalServerError().finish();
            return Err(err);
        }
        Ok(x) => x,
    };

    match add_user(&client).await {
        Err(err) => {
            eprintln!("Add users error: {}", err);
            // HttpResponse::InternalServerError().finish()
            return Err(err);
        }
        // Ok(_) => HttpResponse::Ok().body("Mock User Created"),
        Ok(_) => {
            println!("User created");
            return Ok(());
        }
    }
}
