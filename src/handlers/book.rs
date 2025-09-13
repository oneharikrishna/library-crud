use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::*;
use crate::models::book::Book;
use serde::{Deserialize};

#[get("/books")]
async fn get_books(pool: web::Data<MySqlPool>) -> impl Responder {
    let books = sqlx::query_as::<_,Book>(
        r#"select book_id, title, author, genre, published_year, status from books"#
        )
        .fetch_all(pool.get_ref())
        .await;
    match books {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/books/{id}")]
async fn get_book_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let book_id = id.into_inner();
    let book = sqlx::query_as::<_,Book>("SELECT * FROM books WHERE book_id = ?")
        .bind(book_id)
        .fetch_one(pool.get_ref())
        .await;
    match book {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
pub struct PostRequestBook {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub published_year: String
}
#[post("/books")]
async fn add_book(pool: web::Data<MySqlPool>, book_data: web::Json<PostRequestBook>) -> impl Responder {
    let query_result = sqlx::query(
            "INSERT INTO books (title, author, genre, published_year) VALUES(?, ?, ?, ?)")
            .bind(book_data.title.clone())
            .bind(book_data.author.clone())
            .bind(book_data.genre.clone())
            .bind(book_data.published_year.clone())
            .execute(pool.get_ref())
            .await;
    
    match query_result {
        Ok(result) => HttpResponse::Ok().body(format!("{} added to the library. Id is {}",book_data.title,result.last_insert_id())),
        Err(err) => HttpResponse::InternalServerError().body(format!("{}",err.to_string())),
    }

}

#[derive(Deserialize)]
struct PutRequestBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub published_year: Option<String>,
    pub status: Option<String>
}
#[put("/books/{id}")]
async fn update_book(pool: web::Data<MySqlPool>, update_data: web::Json<PutRequestBook>, id: web::Path<i32>) -> impl Responder {
    let payload = update_data.into_inner();
    let mut query: String = String::from("UPDATE books SET ");
    let mut values: Vec<String> = Vec::new();
    let book_id = id.into_inner();
    if let Some(title) = &payload.title {
        query.push_str("title = ?,");
        values.push(title.to_string());
    }
    if let Some(author) = &payload.author {
        query.push_str("author = ?,");
        values.push(author.to_string());
    }
    if let Some(genre) = &payload.genre {
        query.push_str("genre = ?,");
        values.push(genre.to_string());
    }
    if let Some(published_year) = &payload.published_year {
        query.push_str("published_year = ?,");
        values.push(published_year.to_string());
    }
    if let Some(status) = &payload.status {
        query.push_str("status = ?,");
        values.push(status.to_string());
    }
    query = query.trim_end_matches(',').to_string();
    query.push_str(" WHERE book_id = ?;");
    let mut q = sqlx::query(&query);
    for v in values {
        q = q.bind(v);
    }
    q = q.bind(book_id);
    let result = q.execute(pool.get_ref()).await;

    match result {
        Ok(_result) => HttpResponse::Ok().body(format!("book {} is updated",book_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

#[delete("/books/{id}")]
async fn delete_book_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let book_id = id.into_inner();
    let result = sqlx::query("DELETE FROM books WHERE book_id = ?")
        .bind(book_id)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_res) => HttpResponse::Ok().body(format!("book {} removed from library",book_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub fn book_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books)
        .service(get_book_by_id)
        .service(add_book)
        .service(update_book)
        .service(delete_book_by_id);
}