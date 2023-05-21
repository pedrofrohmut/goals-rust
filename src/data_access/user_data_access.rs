use tokio_postgres::Error;

pub async fn add_user(client: &tokio_postgres::Client) -> Result<(), Error>
{
    let str = "INSERT INTO users (name, email, phone, password_hash) VALUES ($1, $2, $3, $4)";
    let stm = client.prepare(str).await?;

    let name = "John Doe";
    let email = "john@doe.com";
    let phone = "123-456-7890";
    let password_hash = "PASSWORD_HASH";

    client.execute(&stm, &[&name, &email, &phone, &password_hash]).await?;

    Ok(())
}
