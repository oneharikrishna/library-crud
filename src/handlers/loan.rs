use serde::{Serialize, Deserialize};
use sqlx::*;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use reqwest::{Client};
use chrono::{Local, Months};
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
    let result = sqlx::query_as::<_,Loan>("SELECT * FROM loans WHERE loan_id = ?")
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
#[derive(Serialize)]
struct PutBookStatus {
    status: String
}
#[derive(Serialize)]
struct CustomResponse {
    loan_id: u64,
    status: String
}
#[post("/loans")]
async fn add_loan(pool: web::Data<MySqlPool>, loan_data: web::Json<PostRequestLoan>) -> impl Responder {
    let client = Client::new();
    let response = client.get(format!("http://localhost:8080/books/{}",loan_data.book_id.clone()))
        .send()
        .await;
    match response {
        Ok(resp) => {
            let json_content = resp.json::<Book>().await;
            match json_content {
                Ok(res) => {
                    if res.status == "available" {
                        let payload = PutBookStatus {
                            status: "issued".to_string()
                        };
                        let book_update_status = client
                            .put(format!("http://localhost:8080/books/{}",loan_data.book_id.clone()))
                            .json(&payload)
                            .send()
                            .await
                            .unwrap();
                        match book_update_status.status() {
                            _ok => {
                                let now = Local::now();
                                let one_month_later = now + Months::new(1);
                                let loan = sqlx::query("INSERT INTO loans(book_id, member_id, issue_date, return_date) VALUES(?,?,?,?)")
                                    .bind(loan_data.book_id.clone()) 
                                    .bind(loan_data.member_id.clone())
                                    .bind(now.format("%Y-%m-%d").to_string())
                                    .bind(one_month_later.format("%Y-%m-%d").to_string())
                                    .execute(pool.get_ref())
                                    .await;
                                match loan {
                                    Ok(success) => HttpResponse::Ok().json(CustomResponse {
                                        loan_id: success.last_insert_id(),
                                        status: "active".to_string()
                                    }),
                                    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
                                }
                            }
                            // _other => HttpResponse::InternalServerError().body("Something happened")
                        }
                    }
                    else {
                        HttpResponse::Ok().body("Book already issued")
                    }
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string())
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
struct PutRequestLoan {
    return_date: Option<String>,
    status: Option<String>
}
#[put("/loans/{id}")]
async fn update_loan_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>, update_data: web::Json<PutRequestLoan>) -> impl Responder {
    let loan_id = id.into_inner();
    let payload = update_data.into_inner();
    if let Some(return_date) = &payload.return_date {
        let result = sqlx::query("UPDATE loans SET return_date = ? WHERE loan_id = ?")
            .bind(return_date)
            .bind(loan_id)
            .execute(pool.get_ref())
            .await;
        return match result {
            Ok(_res) => HttpResponse::Ok().body(format!("Loan {}, return date updated",loan_id)),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        };
    }
    if let Some(status) = &payload.status {
        let mut transaction = pool.begin().await.unwrap();
        let today = Local::now().format("%y-%m-%d").to_string();
        sqlx::query("UPDATE loans SET status = ?, return_date = ? WHERE loan_id = ?")
            .bind(status)
            .bind(today)
            .bind(loan_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
        sqlx::query("UPDATE books SET status = 'available' WHERE book_id = (SELECT book_id FROM loans WHERE loan_id = ?);")
            .bind(loan_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
        let result = transaction.commit().await;
        return match result {
            Ok(_res) => HttpResponse::Ok().body(format!("Loan {}, status changed",loan_id)),
            Err(err) => HttpResponse::Ok().body(err.to_string())
        };
    }
    HttpResponse::InternalServerError().body("Expected a status or return_date in JSON body")
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
        .service(update_loan_by_id)
        .service(delete_loan_by_id);
}