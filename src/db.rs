use tokio_postgres::{NoTls, Error};

pub async fn establish_connection() -> Result<tokio_postgres::Client, Error>
{
    let connection_string = "host=localhost user=didorgas password=1234 dbname=goals_db";

    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("Connection error: {}", err);
        }
    });

    Ok(client)
}
