use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};

#[post("/api/users")]
async fn signup_route(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Sign Up")
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

#[post("/api/goals")]
async fn add_goal_route(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Add Goal")
}

#[get("/api/goals")]
async fn get_goals_route() -> impl Responder
{
    HttpResponse::Ok().body("Get Goals")
}

#[delete("/api/goals/{goalId}")]
async fn delete_goal_route(path: web::Path<(String)>) -> impl Responder
{
    HttpResponse::Ok().body("Delete goal")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
        App::new()
            .service(signup_route)
            .service(signin_route)
            .service(verify_token_route)
            .service(add_goal_route)
            .service(get_goals_route)
            .service(delete_goal_route)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
