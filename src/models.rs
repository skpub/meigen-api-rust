use crate::schema::posts;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: Uuid,
    
}