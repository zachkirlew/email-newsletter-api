use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (name, email, subscribed_at) VALUES ($1, $2, $3)
        "#,
        form.name,
        form.email,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError()
        }
    }
}
