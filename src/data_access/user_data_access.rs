use std::fmt::Display;

use tokio_postgres::Client;
use uuid::Uuid;

use crate::{entities::user::User, errors::user_errors::InvalidUserError};

pub enum UserDataAccessError {
    DbError(tokio_postgres::Error),
    MappingError(InvalidUserError),
    ParameterError(String),
}

impl Display for UserDataAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserDataAccessError::DbError(err) => write!(f, "{}", err),
            UserDataAccessError::MappingError(err) => write!(f, "{}", err),
            UserDataAccessError::ParameterError(err) => write!(f, "{}", err),
        }
    }
}

pub async fn add_user(client: &Client, user: &User) -> Result<(), UserDataAccessError> {
    let sql = "
        INSERT INTO users
            (name, email, phone, password_hash)
        VALUES
            ($1, $2, $3, $4)";

    let name = user.get_name();
    let email = user.get_email();
    let phone = user.get_phone();
    let password_hash = user.get_password_hash();

    let stm = client
        .prepare(sql)
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    client
        .execute(&stm, &[&name, &email, &phone, &password_hash])
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    Ok(())
}

pub async fn find_user_by_email(
    client: &Client,
    email: &str,
) -> Result<Option<User>, UserDataAccessError> {
    let str = "SELECT * FROM users WHERE email = $1";

    let stm = client
        .prepare(str)
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    let rows = client
        .query(&stm, &[&email])
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    if rows.len() == 0 {
        return Ok(None);
    }

    let id = rows[0].try_get::<_, Uuid>("id").unwrap_or_default();
    let name = rows[0].try_get::<_, String>("name").unwrap_or_default();
    let email = rows[0].try_get::<_, String>("email").unwrap_or_default();
    let password_hash = rows[0]
        .try_get::<_, String>("password_hash")
        .unwrap_or_default();
    let phone = rows[0].try_get::<_, String>("phone").unwrap_or_default();

    let user = User::from_db_fields(&id, &name, &email, &password_hash, &phone)
        .map_err(|err| UserDataAccessError::MappingError(err))?;

    Ok(Some(user))
}

pub async fn find_user_by_id(
    client: &Client,
    id: &str,
) -> Result<Option<User>, UserDataAccessError> {
    let str = "SELECT * FROM users WHERE id = $1";

    let stm = client
        .prepare(str)
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    let id =
        Uuid::parse_str(&id).map_err(|err| UserDataAccessError::ParameterError(err.to_string()))?;

    let rows = client
        .query(&stm, &[&id])
        .await
        .map_err(|err| UserDataAccessError::DbError(err))?;

    if rows.len() == 0 {
        return Ok(None);
    }

    let id = rows[0].try_get::<_, Uuid>("id").unwrap_or_default();
    let name = rows[0].try_get::<_, String>("name").unwrap_or_default();
    let email = rows[0].try_get::<_, String>("email").unwrap_or_default();
    let password_hash = rows[0]
        .try_get::<_, String>("password_hash")
        .unwrap_or_default();
    let phone = rows[0].try_get::<_, String>("phone").unwrap_or_default();

    let user = User::from_db_fields(&id, &name, &email, &password_hash, &phone)
        .map_err(|err| UserDataAccessError::MappingError(err))?;

    Ok(Some(user))
}
