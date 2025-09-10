use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Loan {
    pub loan_id: i32,
    pub book_id: i32,
    pub member_id: i32,
    pub issue_date: String,
    pub return_date: String,
    pub status: String
}