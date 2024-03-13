use email_newsletter_api::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[sqlx::test]
async fn health_check_works(pool: PgPool) {
    let test_app = spawn_app(pool);

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", test_app.await.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(pool: PgPool) {
    let app = spawn_app(pool).await;

    let client = reqwest::Client::new();
    let body = "name=Zach%20K&email=zachkirlew%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "Zach K");
    assert_eq!(saved.email, "zachkirlew@gmail.com");
}

#[sqlx::test]
async fn subscribe_returns_a_400_when_data_is_missing(pool: PgPool) {
    let test_app = spawn_app(pool).await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("", "missing both name and email"),
        ("name=Zach%20Kirlew", "missing email"),
        ("email%3Dzachkirlew%40gmail.com", "missing name"),
    ];

    for (invalid_body, err_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", test_app.address))
            .body(invalid_body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 status when the payload was {}",
            err_msg
        );
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app(pool: PgPool) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Port 0 will trigger an OS scan for an available port which will then be bound to the application.
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: pool,
    }
}
