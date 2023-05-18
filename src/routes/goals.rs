use actix_web::{get, post, delete, web, HttpResponse, Responder};

#[post("/api/goals")]
async fn add_goal_route(_req_body: String) -> impl Responder
{
    HttpResponse::Ok().body("Add Goal")
}

#[get("/api/goals")]
async fn get_goals_route() -> impl Responder
{
    HttpResponse::Ok().body("Get Goals")
}

#[delete("/api/goals/{goalId}")]
async fn delete_goal_route(_path: web::Path<String>) -> impl Responder
{
    HttpResponse::Ok().body("Delete goal")
}
