use tokio_postgres::Client;

use crate::{
    data_access::{goal_data_access, user_data_access::find_user_by_id},
    db::establish_connection,
    entities::goal::Goal,
};

pub enum DeleteGoalError {
    DatabaseError(String),
    UserNotFoundError(String),
    InvalidRequestError(String),
}

pub async fn execute(goal_id: String, user_id: String) -> Result<(), DeleteGoalError> {
    let client = get_connected_client().await?;
    find_user(&client, &user_id).await?;
    Goal::validate_id(&goal_id)
        .map_err(|err| DeleteGoalError::InvalidRequestError(err.to_string()))?;
    delete_goal(&client, &goal_id).await?;
    Ok(())
}

async fn get_connected_client() -> Result<Client, DeleteGoalError> {
    let client = establish_connection()
        .await
        .map_err(|err| DeleteGoalError::DatabaseError(err.to_string()))?;
    Ok(client)
}

async fn find_user(client: &Client, user_id: &str) -> Result<(), DeleteGoalError> {
    let opt_user = find_user_by_id(&client, &user_id)
        .await
        .map_err(|err| DeleteGoalError::DatabaseError(err.to_string()))?;

    match opt_user {
        None => {
            return Err(DeleteGoalError::UserNotFoundError(
                "User not found with the id present in the authorization headers".to_string(),
            ))
        }
        Some(_) => Ok(()),
    }
}

async fn delete_goal(client: &Client, goal_id: &str) -> Result<(), DeleteGoalError> {
    goal_data_access::delete_goal(&client, &goal_id)
        .await
        .map_err(|err| DeleteGoalError::DatabaseError(err.to_string()))?;
    Ok(())
}
