use actix_web::{App, HttpServer};

use crate::routes::users::*;
use crate::routes::goals::*;

pub mod routes;

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
