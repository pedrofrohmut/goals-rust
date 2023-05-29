use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    entities::user::{CreateUserDto, CredentialsDto},
    use_cases::users::{
        sign_in::{self, SignInError},
        sign_up::{self, SignUpError},
        verify_token::{self, VerifyTokenError},
    },
};

#[post("/api/users")]
async fn signup_route(req_body: web::Json<CreateUserDto>) -> impl Responder {
    match sign_up::execute(req_body.into_inner()).await {
        Err(err) => match err {
            SignUpError::RequestValidationError(validation_err) => {
                HttpResponse::BadRequest().body(validation_err.to_string())
            }
            SignUpError::HashPasswordError(hash_err) => {
                HttpResponse::InternalServerError().body(format!("Server error: {}", hash_err))
            }
            SignUpError::EmailAlreadyTakenError(email_err) => {
                HttpResponse::BadRequest().body(email_err.to_string())
            }
            SignUpError::DbError(db_err) => {
                HttpResponse::InternalServerError().body(format!("Server error: {}", db_err))
            }
        },
        Ok(_) => HttpResponse::Created().body("User created"),
    }
}

#[post("/api/users/signin")]
async fn signin_route(req_body: web::Json<CredentialsDto>) -> impl Responder {
    match sign_in::execute(req_body.into_inner()).await {
        Err(error) => match error {
            SignInError::InvalidRequestError(req_err) => {
                HttpResponse::BadRequest().body(req_err.to_string())
            }
            SignInError::DatabaseError(db_err) => {
                HttpResponse::InternalServerError().body(db_err.to_string())
            }
            SignInError::UserNotFound(err) => HttpResponse::NotFound().body(err.to_string()),

            SignInError::PasswordAndHashDontMatchError(err) => {
                HttpResponse::BadRequest().body(err.to_string())
            }
            SignInError::GenerateJwtError(err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        },
        Ok(signed_user) => HttpResponse::Ok().json(signed_user),
    }
}

#[get("/api/users/verify")]
async fn verify_token_route(req: HttpRequest) -> impl Responder {
    let token = match req.headers().get("authorization") {
        None => return HttpResponse::BadRequest().body("Missing JWT in authorization headers"),
        Some(auth_header) => {
            let auth_header = String::from(auth_header.to_str().unwrap());
            let token = auth_header.split(" ").collect::<Vec<&str>>()[1].to_string();
            token
        }
    };
    match verify_token::execute(token).await {
        Err(error) => match error {
            VerifyTokenError::DecodeTokenError(err_msg) => HttpResponse::BadRequest().body(err_msg),

            VerifyTokenError::DatabaseError(err_msg) => {
                HttpResponse::InternalServerError().body(err_msg)
            }
            VerifyTokenError::UserNotFound(err_msg) => HttpResponse::NotFound().body(err_msg),
        },
        Ok(_) => HttpResponse::Ok().body("Token Verified"),
    }
}
