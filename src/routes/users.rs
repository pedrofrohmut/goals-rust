use actix_web::{get, post, HttpResponse, Responder};

#[post("/api/users")]
async fn signup_route(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Sign Up Route")
}

#[post("/api/users/signin")]
async fn signin_route(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Sign In")
}

#[get("/api/users/verify")]
async fn verify_token_route() -> impl Responder
{
    HttpResponse::Ok().body("Verify Token")
}
