use std::{fmt::{Display, self}, error::Error};

#[derive(Debug)]
pub struct InvalidUserError(String);

impl InvalidUserError {
    pub fn new(message: Option<String>) -> InvalidUserError
    {
        match message {
            Some(msg) => InvalidUserError(msg),
            None => InvalidUserError("Err: User is invalid".into()),
        }
    }
}

impl Display for InvalidUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidUserError {}
