use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn connect_db() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DB_URL")
        .expect("DB_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
