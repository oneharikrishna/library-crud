use actix_web::{web, HttpServer, App};
use dotenvy::dotenv;

mod db;
mod handlers;
use handlers::{book, member, loan};
mod models;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = db::get_db_pool().await;
        
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(book::book_services)
            .configure(member::member_services)
            .configure(loan::loan_services)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}