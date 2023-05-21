use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::use_cases::users::sign_up;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserBody {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[post("/api/users")]
async fn signup_route(create_user_body: web::Json<CreateUserBody>) -> impl Responder
{
    let _res = sign_up::execute(create_user_body.into_inner()).await;

    HttpResponse::Created().body("User created.")
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
