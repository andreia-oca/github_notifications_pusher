use std::env;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[get("/github/notifications")]
async fn github_notifications() -> impl Responder {
    let _github_token = env::var("GITHUB_USER_TOKEN").expect("GITHUB_USER_TOKEN is not set");

    HttpResponse::Ok().body("ok")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
    );
}
