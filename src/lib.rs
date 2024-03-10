use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(healthcheck))
    })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}