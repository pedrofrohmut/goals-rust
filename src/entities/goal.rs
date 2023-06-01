use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::goal_errors::InvalidGoalError;

use super::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGoalDto {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoalDto {
    pub id: String,
    pub text: String,
    pub user_id: String,
}

pub struct Goal {
    id: String,
    text: String,
    user_id: String,
}

impl Goal {
    pub fn new() -> Goal {
        Goal {
            id: String::from("NO _ID"),
            text: String::from("NO_TEXT"),
            user_id: String::from("NO_USER_ID"),
        }
    }

    pub fn validate_id(id: &str) -> Result<(), InvalidGoalError> {
        match Uuid::parse_str(id) {
            Err(_) => Err(InvalidGoalError::new(Some(
                "Goal id is not a valid UUID".to_string(),
            ))),
            Ok(_) => Ok(()),
        }
    }

    pub fn validate_text(text: &str) -> Result<(), InvalidGoalError> {
        if text.is_empty() {
            return Err(InvalidGoalError::new(Some(String::from(
                "Goal text is required and cannot be empty",
            ))));
        }
        Ok(())
    }

    pub fn set_id(&mut self, id: String) -> Result<(), InvalidGoalError> {
        match Goal::validate_id(&id) {
            Err(err) => Err(err),
            Ok(_) => {
                self.id = id;
                Ok(())
            }
        }
    }

    pub fn set_text(&mut self, text: String) -> Result<(), InvalidGoalError> {
        match Goal::validate_text(&text) {
            Err(err) => Err(err),
            Ok(_) => {
                self.text = text;
                Ok(())
            }
        }
    }

    pub fn set_user_id(&mut self, user_id: String) -> Result<(), InvalidGoalError> {
        match User::validate_id(&user_id) {
            Err(err) => Err(InvalidGoalError::new(Some(err.to_string()))),
            Ok(_) => {
                self.user_id = user_id;
                Ok(())
            }
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn from_create_goal_dto(
        create_goal: CreateGoalDto,
        user_id: &str,
    ) -> Result<Goal, InvalidGoalError> {
        let mut goal = Goal::new();
        if let Err(err) = goal.set_text(create_goal.text) {
            return Err(err);
        }
        if let Err(err) = goal.set_user_id(user_id.to_string()) {
            return Err(err);
        }
        Ok(goal)
    }
}
