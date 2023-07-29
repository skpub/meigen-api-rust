use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use pwhash::bcrypt;

use crate::{errors::MyError, models::User};

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = format!("SELECT {} FROM meigen.users;", &User::sql_table_fields());
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
    let stmt = format!("INSERT INTO meigen.users VALUES($1, $2) RETURNING {};", &User::sql_table_fields());
    let pw = bcrypt::hash(user_info.pwhash).unwrap().to_string();
    client
        .query(
            &stmt,
            &[
                &user_info.name,
                &pw
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}
