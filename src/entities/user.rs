use serde::{Deserialize, Serialize};

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

    pub fn validate_name(name: &str) -> bool { true }

    pub fn validate_email(email: &str) -> bool { true }

    pub fn validate_password(password: &str) -> bool { true }

    pub fn validate_phone(phone: &str) -> bool { true }

    pub fn set_name(mut self, name: String) -> Self
    {
        User::validate_name(&name);
        self.name = name;
        self
    }

    pub fn set_email(mut self, email: String) -> Self
    {
        User::validate_email(&email);
        self.email = email;
        self
    }

    pub fn set_password(mut self, password: String) -> Self
    {
        User::validate_password(&password);
        self.password = password;
        self
    }

    pub fn set_phone(mut self, phone: String) -> Self
    {
        User::validate_phone(&phone);
        self.phone = phone;
        self
    }

    pub fn from_create_user_dto(create_user: CreateUserDto) -> User
    {
        User::new()
            .set_name(create_user.name)
            .set_email(create_user.email)
            .set_password(create_user.password)
            .set_phone(create_user.phone)
    }
}
