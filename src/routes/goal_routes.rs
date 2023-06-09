use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    entities::goal::CreateGoalDto,
    use_cases::goals::{
        create_goal::{self, CreateGoalError},
        delete_goal::{self, DeleteGoalError},
        get_all_goals::{self, GetAllGoalsError},
    },
    utils::routes_utils::extract_user_id_from_headers,
};

const JWT_MESSAGE: &'static str = "Missing or invalid JWT in authorization headers";

#[post("/api/goals")]
pub async fn add_goal_route(
    req_body: web::Json<CreateGoalDto>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match extract_user_id_from_headers(&req) {
        None => return HttpResponse::BadRequest().body(JWT_MESSAGE),
        Some(id) => id,
    };

    match create_goal::execute(req_body.into_inner(), user_id).await {
        Err(error) => match error {
            CreateGoalError::InvalidRequestError(err_msg) => {
                HttpResponse::BadRequest().body(err_msg)
            }
            CreateGoalError::UserNotFoundError(err_msg) => HttpResponse::NotFound().body(err_msg),

            CreateGoalError::DatabaseError(err_msg) => {
                HttpResponse::InternalServerError().body(err_msg)
            }
        },
        Ok(_) => HttpResponse::Created().body("Goal created"),
    }
}

#[get("/api/goals")]
async fn get_goals_route(req: HttpRequest) -> impl Responder {
    let user_id = match extract_user_id_from_headers(&req) {
        None => return HttpResponse::BadRequest().body(JWT_MESSAGE),
        Some(id) => id,
    };

    match get_all_goals::execute(user_id).await {
        Err(error) => match error {
            GetAllGoalsError::DatabaseError(err_msg) => {
                HttpResponse::InternalServerError().body(err_msg)
            }
            GetAllGoalsError::UserNotFoundError(err_msg) => HttpResponse::NotFound().body(err_msg),
        },
        Ok(goals) => HttpResponse::Ok().json(goals),
    }
}

#[delete("/api/goals/{goalId}")]
async fn delete_goal_route(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = match extract_user_id_from_headers(&req) {
        None => return HttpResponse::BadRequest().body(JWT_MESSAGE),
        Some(id) => id,
    };

    let goal_id = path.into_inner();

    match delete_goal::execute(goal_id, user_id).await {
        Err(error) => match error {
            DeleteGoalError::DatabaseError(err_msg) => {
                HttpResponse::InternalServerError().body(err_msg)
            }

            DeleteGoalError::UserNotFoundError(err_msg) => HttpResponse::NotFound().body(err_msg),

            DeleteGoalError::InvalidRequestError(err_msg) => {
                HttpResponse::BadRequest().body(err_msg)
            }
        },
        Ok(_) => HttpResponse::NoContent().body(""),
    }
}
