use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{ PgPool};

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}


#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, db_pool),
fields(
subscriber_email = % form.email,
subscriber_name = % form.name
)
)
]
pub async fn subscribe(form: Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&form, &db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
name = "Saving new subscriber details in the database", skip(form, db_pool)
)]
pub async fn insert_subscriber(form: &FormData, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (name, email, subscribed_at) VALUES ($1, $2, $3)
        "#,
        form.name,
        form.email,
        Utc::now()
    )
        .execute(db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}



