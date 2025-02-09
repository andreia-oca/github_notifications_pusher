use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::{ok, Ready};
use std::{future::{Future}, pin::Pin, task::{Context, Poll}, time::Instant};
use log::info;
use std::rc::Rc;

// Middleware struct
pub struct RequestLogger;

// Middleware transform
impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware { service: Rc::new(service) })
    }
}

// Middleware struct for request processing
pub struct RequestLoggerMiddleware<S> {
    service: Rc<S>,
}

// Middleware request handling
impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let start_time = Instant::now();
        let service = self.service.clone();

        info!("STARTING {} {}", method, path);

        let fut = service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed().as_millis();
            let status = res.status();

            info!("FINISHED {} {} -> {} in {}ms", method, path, status, duration);

            Ok(res)
        })
    }
}
