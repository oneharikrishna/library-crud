use actix_web::{get, post, delete, put, HttpResponse, web, Responder};
use serde::{Deserialize};
use sqlx::*;
use crate::models::member::Member;

#[get("/members")]
async fn get_members(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_,Member>("SELECT * FROM members")
        .fetch_all(pool.get_ref())
        .await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/members/{id}")]
async fn get_member_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let member_id = id.into_inner();
    let result = sqlx::query_as::<_,Member>("SELECT * FROM members WHERE member_id = ?")
        .bind(member_id)
        .fetch_one(pool.get_ref())
        .await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
struct PostRequestMember {
    name: String,
    phone: String,
    email: String,
    join_date: String,
}
#[post("/members")]
async fn add_member(pool: web::Data<MySqlPool>, payload: web::Json<PostRequestMember>) -> impl Responder {
    let result = sqlx::query("INSERT INTO members (name, phone, email, join_date) VALUES(?, ?, ?, ?)")
        .bind(payload.name.clone())
        .bind(payload.phone.clone())
        .bind(payload.email.clone())
        .bind(payload.join_date.clone())
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(res) => HttpResponse::Ok().body(format!("{} added to the members. Id is {}",payload.name,res.last_insert_id())),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[derive(Deserialize)]
struct PutRequestMember {
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    join_date: Option<String>,
}
#[put("/members/{id}")]
async fn update_member(pool: web::Data<MySqlPool>, update_data: web::Json<PutRequestMember>, id: web::Path<i32>) -> impl Responder {
    let payload = update_data.into_inner();
    let mut query: String = String::from("UPDATE members SET ");
    let mut values: Vec<String> = Vec::new();
    let member_id = id.into_inner();

    if let Some(name) = &payload.name {
        query.push_str(" name = ?,");
        values.push(name.to_string());
    }
    if let Some(phone) = &payload.phone {
        query.push_str(" phone = ?,");
        values.push(phone.to_string());
    }
    if let Some(email) = &payload.email {
        query.push_str(" email = ?,");
        values.push(email.to_string());
    }
    if let Some(join_date) = &payload.join_date {
        query.push_str(" join_date = ?,");
        values.push(join_date.to_string());
    }
    query = query.trim_end_matches(',').to_string();
    query.push_str(" WHERE member_id = ?;");
    let mut q = sqlx::query(&query);
    for v in values {
        q = q.bind(v);
    }
    q = q.bind(member_id);
    let result = q.execute(pool.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(format!("member {} is updated",member_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[delete("/members/{id}")]
async fn delete_member_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let member_id = id.into_inner();
    let result = sqlx::query("DELETE FROM members WHERE member_id = ?;")
        .bind(member_id)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_res) => HttpResponse::Ok().body(format!("Member {} removed from members",member_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub fn member_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_members)
        .service(get_member_by_id)
        .service(add_member)
        .service(update_member)
        .service(delete_member_by_id);
}