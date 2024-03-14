use email_newsletter_api::configuration;
use email_newsletter_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use email_newsletter_api::telemetry::{get_tracing_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tracing_subscriber = get_tracing_subscriber("email-newsletter-api".into(), "info".into());
    init_subscriber(tracing_subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
