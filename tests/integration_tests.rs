#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use github_notification_pusher::init; // Now it should resolve correctly

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().configure(init)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, "Rust service prototype");
    }

    #[actix_web::test]
    async fn test_healthcheck() {
        let app = test::init_service(App::new().configure(init)).await;
        let req = test::TestRequest::get().uri("/healthcheck").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, "ok");
    }
}
