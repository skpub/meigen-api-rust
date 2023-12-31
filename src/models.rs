use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use pwhash::bcrypt;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table="users")]
pub struct User {
    pub name: String,
    pub pwhash: String,
}
