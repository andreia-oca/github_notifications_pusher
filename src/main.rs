use actix_web::{App, HttpServer};
use github_notification_pusher::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
