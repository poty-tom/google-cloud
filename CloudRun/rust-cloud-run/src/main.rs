use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use env_logger::Env;
#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // host server setting
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // add middleware
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}