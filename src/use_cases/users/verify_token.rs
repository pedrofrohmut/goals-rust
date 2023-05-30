use tokio_postgres::Client;

use crate::{
    data_access::user_data_access::find_user_by_id, db::establish_connection,
    services::auth_services::validate_and_get_id_from_token,
};

pub enum VerifyTokenError {
    DecodeTokenError(String),
    DatabaseError(String),
    UserNotFound(String),
}

pub async fn execute(token: String) -> Result<(), VerifyTokenError> {
    let user_id = validate_and_get_id_from_token(&token)
        .map_err(|err| VerifyTokenError::DecodeTokenError(err.to_string()))?;

    let client = get_connected_client().await?;

    find_user(&client, &user_id).await?;

    Ok(())
}

async fn find_user(client: &Client, user_id: &str) -> Result<(), VerifyTokenError> {
    let found_user = find_user_by_id(&client, &user_id)
        .await
        .map_err(|err| VerifyTokenError::DatabaseError(err.to_string()))?;

    match found_user {
        None => Err(VerifyTokenError::UserNotFound(format!(
            "User not found for the id: {}",
            &user_id
        ))),
        Some(_user) => Ok(()),
    }
}

async fn get_connected_client() -> Result<tokio_postgres::Client, VerifyTokenError> {
    // Calls up db util to get a connected client
    let client = establish_connection().await.map_err(|err| {
        eprintln!("Client connection error: {}", err);
        VerifyTokenError::DatabaseError(err.to_string())
    })?;

    Ok(client)
}
