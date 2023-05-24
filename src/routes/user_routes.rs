use actix_web::{get, post, HttpResponse, Responder, web};

use crate::{use_cases::users::sign_up::{self, SignUpError}, entities::user::CreateUserDto};

#[post("/api/users")]
async fn signup_route(req_body: web::Json<CreateUserDto>) -> impl Responder
{
    match sign_up::execute(req_body.into_inner()).await {
        Err(err) => match err {
            SignUpError::RequestValidationError(validation_err) => {
                HttpResponse::BadRequest().body(validation_err.to_string())
            }
            SignUpError::HashPasswordError(hash_err) => {
                HttpResponse::InternalServerError().body(format!("Server error: {}", hash_err))
            }
            SignUpError::DbError(db_err) => {
                HttpResponse::InternalServerError().body(format!("Server error: {}", db_err))
            }
        },
        Ok(_) => HttpResponse::Created().body("User created"),
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
