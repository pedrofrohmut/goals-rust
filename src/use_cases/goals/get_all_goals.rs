use tokio_postgres::Client;

use crate::{
    data_access::{goal_data_access::find_all_goals, user_data_access::find_user_by_id},
    db::establish_connection,
    entities::goal::{Goal, GoalDto},
};

pub enum GetAllGoalsError {
    DatabaseError(String),
    UserNotFoundError(String),
}

pub async fn execute(user_id: String) -> Result<Vec<GoalDto>, GetAllGoalsError> {
    let client = get_connected_client().await?;
    find_user(&client, &user_id).await?;
    let goals_db = find_goals(&client, &user_id).await?;
    let goals = map_to_dtos(goals_db);
    Ok(goals)
}

async fn get_connected_client() -> Result<Client, GetAllGoalsError> {
    let client = establish_connection()
        .await
        .map_err(|err| GetAllGoalsError::DatabaseError(err.to_string()))?;
    Ok(client)
}

async fn find_user(client: &Client, user_id: &str) -> Result<(), GetAllGoalsError> {
    let opt_user = find_user_by_id(&client, &user_id)
        .await
        .map_err(|err| GetAllGoalsError::DatabaseError(err.to_string()))?;

    match opt_user {
        None => {
            return Err(GetAllGoalsError::UserNotFoundError(
                "User not found with the id present in the authorization headers".to_string(),
            ))
        }
        Some(_) => Ok(()),
    }
}

async fn find_goals(client: &Client, user_id: &str) -> Result<Vec<Goal>, GetAllGoalsError> {
    let goals = find_all_goals(&client, &user_id)
        .await
        .map_err(|err| GetAllGoalsError::DatabaseError(err.to_string()))?;
    Ok(goals)
}

fn map_to_dtos(goals_db: Vec<Goal>) -> Vec<GoalDto> {
    let mut goals_dto = Vec::new();
    for goal_db in goals_db {
        let goal_dto = GoalDto {
            id: goal_db.get_id(),
            text: goal_db.get_text(),
            user_id: goal_db.get_user_id(),
        };
        goals_dto.push(goal_dto);
    }
    goals_dto
}
