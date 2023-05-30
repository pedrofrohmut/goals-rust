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
