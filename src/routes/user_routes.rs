use actix_web::{get, post, HttpResponse, Responder, web};

use crate::{use_cases::users::sign_up, entities::user::CreateUserDto};

#[post("/api/users")]
async fn signup_route(req_body: web::Json<CreateUserDto>) -> impl Responder
{
    let _res = sign_up::execute(req_body.into_inner()).await;

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
