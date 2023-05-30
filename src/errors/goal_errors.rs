use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct InvalidGoalError(String);

impl InvalidGoalError {
    pub fn new(message: Option<String>) -> InvalidGoalError {
        match message {
            None => InvalidGoalError("Err: Goal is invalid".into()),
            Some(msg) => InvalidGoalError(msg),
        }
    }
}

impl Display for InvalidGoalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidGoalError {}
