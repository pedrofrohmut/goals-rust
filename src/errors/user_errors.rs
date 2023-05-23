use std::{fmt::{Display, self}, error::Error};

#[derive(Debug)]
pub struct InvalidUserErr(String);

impl InvalidUserErr {
    pub fn new(message: Option<String>) -> InvalidUserErr
    {
        match message {
            Some(msg) => InvalidUserErr(msg),
            None => InvalidUserErr("Err: User is invalid".into()),
        }
    }
}

impl Display for InvalidUserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidUserErr {}
