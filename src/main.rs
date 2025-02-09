use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use github_notification_pusher::{init,middleware::RequestLogger};
use env_logger;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let addr: &str = "0.0.0.0:8080";
    info!("Starting server at: {}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(RequestLogger)
            .configure(init)
    })
    .bind(addr)?
    .run()
    .await
}
