use serde::{Deserialize, Serialize};
use uuid::Uuid;
use regex::Regex;

use crate::errors::user_errors::InvalidUserError;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

pub struct User {
    id: String,
    name: String,
    email: String,
    password: String,
    password_hash: String,
    phone: String,
}

impl User {
    fn new() -> User
    {
        User {
            id: String::from(""),
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            password_hash: String::from(""),
            phone: String::from(""),
        }
    }

    pub fn validate_id(id: &str) -> Result<(), InvalidUserError>
    {
        match Uuid::parse_str(id) {
            Err(_) => Err(InvalidUserError::new(Some(String::from("User id is not a valid UUID")))),
            Ok(_) => Ok(())
        }
    }

    pub fn validate_name(name: &str) -> Result<(), InvalidUserError>
    {
        if name.is_empty() {
            return Err(InvalidUserError::new(Some(String::from("User name is required and cannot be blank"))));
        }
        if name.len() < 5 || name.len() > 120 {
            return Err(InvalidUserError::new(Some(String::from("User name must be between 5 and 120 characters long"))));
        }
        Ok(())
    }

    pub fn validate_email(email: &str) -> Result<(), InvalidUserError>
    {
        if email.is_empty() {
            return Err(InvalidUserError::new(Some(String::from("User email is required and cannot be empty"))));
        }
        let pattern = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$";
        let regex = Regex::new(pattern).unwrap();
        if ! regex.is_match(email) {
            return Err(InvalidUserError::new(Some(String::from("User email is not in a valid format"))));
        }
        Ok(())
    }

    pub fn validate_password(password: &str) -> Result<(), InvalidUserError>
    {
        if password.is_empty() {
            return Err(InvalidUserError::new(Some(String::from("User password is required and cannot be empty"))));
        }
        if password.len() < 3 || password.len() > 32 {
            return Err(InvalidUserError::new(Some(String::from("User password must be betweeen 3 and 32 characters long"))));
        }
        Ok(())
    }

    pub fn validate_phone(phone: &str) -> Result<(), InvalidUserError>
    {
        if phone.is_empty() {
            return Err(InvalidUserError::new(Some(String::from("User phone is required and cannot be empty"))));
        }
        let pattern = r"^\d{3}-\d{3}-\d{4}$";
        let regex = Regex::new(pattern).unwrap();
        if ! regex.is_match(phone) {
            return Err(InvalidUserError::new(Some(String::from("User phone is not in a valid phone pattern"))));
        }
        Ok(())
    }

    pub fn set_name(&mut self, name: String) -> Result<(), InvalidUserError>
    {
        match User::validate_name(&name) {
            Err(err) => Err(err),
            Ok(_) => {
                self.name = name;
                Ok(())
            }
        }
    }

    pub fn set_email(&mut self, email: String) -> Result<(), InvalidUserError>
    {
        match User::validate_email(&email) {
            Err(err) => Err(err),
            Ok(_) => {
                self.email = email;
                Ok(())
            }
        }
    }

    pub fn set_password(&mut self, password: String) -> Result<(), InvalidUserError>
    {
        match User::validate_password(&password) {
            Err(err) => Err(err),
            Ok(_) => {
                self.password = password;
                Ok(())
            }
        }
    }

    pub fn set_password_hash(&mut self, password_hash: String)
    {
        self.password_hash = password_hash;
    }

    pub fn set_phone(&mut self, phone: String) -> Result<(), InvalidUserError>
    {
        match User::validate_phone(&phone) {
            Err(err) => Err(err),
            Ok(_) => {
                self.phone = phone;
                Ok(())
            }
        }
    }

    pub fn get_name(&self) -> String { self.name.clone() }

    pub fn get_email(&self) -> String { self.email.clone() }

    pub fn get_password(&self) -> String { self.password.clone() }

    pub fn get_password_hash(&self) -> String { self.password_hash.clone() }

    pub fn get_phone(&self) -> String { self.phone.clone() }

    pub fn from_create_user_dto(create_user: CreateUserDto) -> Result<User, InvalidUserError>
    {
        let mut user = User::new();
        if let Err(err) = user.set_name(create_user.name) {
            return Err(err);
        };
        if let Err(err) = user.set_email(create_user.email) {
            return Err(err);
        };
        if let Err(err) = user.set_password(create_user.password) {
            return Err(err);
        };
        if let Err(err) = user.set_phone(create_user.phone) {
            return Err(err);
        };
        Ok(user)
    }
}
