use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
    );
}
