use std::fmt::{self, Display};

use tokio_postgres::Client;
use uuid::Uuid;

use crate::entities::goal::Goal;

pub enum GoalDataAccessError {
    DatabaseError(String),
    MappingError(String),
    ParameterError(String),
}

impl Display for GoalDataAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoalDataAccessError::DatabaseError(err) => write!(f, "{}", err),
            GoalDataAccessError::MappingError(err) => write!(f, "{}", err),
            GoalDataAccessError::ParameterError(err) => write!(f, "{}", err),
        }
    }
}

pub async fn add_goal(client: &Client, goal: &Goal) -> Result<(), GoalDataAccessError> {
    let sql = "
        INSERT INTO goals
            (text, user_id)
        VALUES
            ($1, $2)";

    let user_id = Uuid::parse_str(&goal.get_user_id())
        .map_err(|err| GoalDataAccessError::ParameterError(err.to_string()))?;
    let text = goal.get_text();

    let stm = client
        .prepare(sql)
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    client
        .execute(&stm, &[&text, &user_id])
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    Ok(())
}

pub async fn find_all_goals(
    client: &Client,
    user_id: &str,
) -> Result<Vec<Goal>, GoalDataAccessError> {
    let sql = "SELECT * FROM goals WHERE user_id = $1";

    let stm = client
        .prepare(sql)
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    let user_id = Uuid::parse_str(user_id)
        .map_err(|err| GoalDataAccessError::ParameterError(err.to_string()))?;

    let rows = client
        .query(&stm, &[&user_id])
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    if rows.len() == 0 {
        return Ok(Vec::new());
    }

    let user_id = user_id.to_string();
    let mut goals = Vec::new();
    for row in rows.iter() {
        let id = row.try_get::<_, Uuid>("id").unwrap_or_default().to_string();
        let text = row.try_get::<_, String>("text").unwrap_or_default();

        let mut goal = Goal::new();
        goal.set_id(id)
            .map_err(|err| GoalDataAccessError::MappingError(err.to_string()))?;
        goal.set_text(text)
            .map_err(|err| GoalDataAccessError::MappingError(err.to_string()))?;
        goal.set_user_id(user_id.clone())
            .map_err(|err| GoalDataAccessError::MappingError(err.to_string()))?;

        goals.push(goal);
    }

    Ok(goals)
}

pub async fn delete_goal(client: &Client, id: &str) -> Result<(), GoalDataAccessError> {
    let sql = "DELETE FROM goals WHERE id = $1";

    let stm = client
        .prepare(sql)
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    let goal_id =
        Uuid::parse_str(id).map_err(|err| GoalDataAccessError::ParameterError(err.to_string()))?;

    client
        .execute(&stm, &[&goal_id])
        .await
        .map_err(|err| GoalDataAccessError::DatabaseError(err.to_string()))?;

    Ok(())
}
