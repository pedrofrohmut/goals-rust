use actix_web::{App, HttpServer};

use crate::routes::goal_routes::*;
use crate::routes::user_routes::*;

mod data_access;
mod entities;
mod errors;
mod routes;
mod services;
mod use_cases;
mod utils;
mod db;

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
