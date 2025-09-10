use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Member {
    pub member_id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub join_date: String,
}