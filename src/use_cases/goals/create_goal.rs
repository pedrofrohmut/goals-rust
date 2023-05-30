use tokio_postgres::Client;

use crate::{
    data_access::{goal_data_access::add_goal, user_data_access::find_user_by_id},
    db::establish_connection,
    entities::{
        goal::{CreateGoalDto, Goal},
        user::User,
    },
};

pub enum CreateGoalError {
    InvalidRequestError(String),
    UserNotFoundError(String),
    DatabaseError(String),
}

pub async fn execute(new_goal: CreateGoalDto, user_id: String) -> Result<(), CreateGoalError> {
    let client = get_connected_client().await?;

    find_user(&client, &user_id).await?;

    let goal = Goal::from_create_goal_dto(new_goal, &user_id)
        .map_err(|err| CreateGoalError::InvalidRequestError(err.to_string()))?;

    add_goal(&client, &goal)
        .await
        .map_err(|err| CreateGoalError::DatabaseError(err.to_string()))?;

    Ok(())
}

async fn find_user(client: &Client, user_id: &str) -> Result<User, CreateGoalError> {
    let opt_user = match find_user_by_id(&client, &user_id).await {
        Err(err) => return Err(CreateGoalError::DatabaseError(err.to_string())),
        Ok(opt_user) => opt_user,
    };

    match opt_user {
        None => Err(CreateGoalError::UserNotFoundError(
            "User not found with the id from token".to_string(),
        )),
        Some(user) => Ok(user),
    }
}

async fn get_connected_client() -> Result<tokio_postgres::Client, CreateGoalError> {
    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        CreateGoalError::DatabaseError(err.to_string())
    })?;

    Ok(client)
}
