use actix_web::{get, post, HttpResponse, Responder};

use crate::db::establish_connection;
use crate::data_access::users::add_user;

#[post("/api/users")]
async fn signup_route(_req_body: String) -> impl Responder
{
    let client = match establish_connection().await {
        Err(err) => {
            eprintln!("Client connection error: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
        Ok(x) => x,
    };

    match add_user(&client).await {
        Err(err) => {
            eprintln!("Add users error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::Ok().body("Mock User Created"),
    }
}

#[post("/api/users/signin")]
async fn signin_route(_req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Sign In")
}

#[get("/api/users/verify")]
async fn verify_token_route() -> impl Responder
{
    HttpResponse::Ok().body("Verify Token")
}
