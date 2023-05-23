use tokio_postgres::Error;

use crate::entities::user::User;

pub async fn add_user(client: &tokio_postgres::Client, user: &User) -> Result<(), Error>
{
    let str = "INSERT INTO users
                   (name, email, phone, password_hash)
               VALUES
                   ($1, $2, $3, $4)";

    let name = user.get_name();
    let email = user.get_email();
    let phone = user.get_phone();
    let password_hash = user.get_password_hash();

    let stm = client.prepare(str).await?;
    client.execute(&stm, &[&name, &email, &phone, &password_hash]).await?;

    Ok(())
}
