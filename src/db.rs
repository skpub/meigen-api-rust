use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::User};

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = format!("SELECT {} FROM testing.users;", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();
    Ok(results)
}

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let stmt = format!("INSERT INTO testing.users VALUES($1, $2) RETURNING {};", &User::sql_table_fields());
    
    client
        .query(
            &stmt,
            &[
                &user_info.name,
                &user_info.pwhash,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}
