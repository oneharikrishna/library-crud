use serde::{Serialize, Deserialize};
use sqlx::{FromRow};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub published_year: String,
    pub status: String
}