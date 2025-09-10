use serde::{Deserialize};
use sqlx::*;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use reqwest::{Client, StatusCode, RequestBuilder};
use crate::models::loan::Loan;
use crate::models::book::Book;

#[get("/loans")]
async fn get_loans(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_,Loan>("SELECT * FROM loans")
        .fetch_all(pool.get_ref())
        .await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/loans/{id}")]
async fn get_loan_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let loan_id = id.into_inner();
    let result = sqlx::query_as::<_,Loan>("SELECT * FROM loand WHERE loan_id = ?")
        .bind(loan_id)
        .fetch_all(pool.get_ref())
        .await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
struct PostRequestLoan {
    book_id: i32,
    member_id: i32
}

#[post("/loans")]
async fn add_loan(pool: web::Data<MySqlPool>, loan_data: web::Json<PostRequestLoan>) -> impl Responder {
    let client = Client::new();
    let result = client.get(format!("http://localhost:8080/books/{}",loan_data.book_id.clone()))
        .send()
        .json();
    println!("{result:#?}");
    match result.status() {
        OK => HttpResponse::Ok().body(" "),
        other => HttpResponse::Ok().body(" ")
    }
}

#[delete("/loans/{id}")]
async fn delete_loan_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let loan_id = id.into_inner();
    let result = sqlx::query("DELETE * FROM loans WHERE loan_id = ?")
        .bind(loan_id)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_res) => HttpResponse::Ok().body(format!("Loan {} deleted from loans record",loan_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub fn loan_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_loans)
        .service(get_loan_by_id)
        .service(add_loan)
        .service(delete_loan_by_id);
}