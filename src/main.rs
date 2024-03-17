use email_newsletter_api::configuration;
use email_newsletter_api::startup::run;
use email_newsletter_api::telemetry::{get_tracing_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tracing_subscriber = get_tracing_subscriber("email-newsletter-api".into(), "info".into());
    init_subscriber(tracing_subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
