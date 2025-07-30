use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, http::header};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;

const API_TOKEN: &str = "my_secret_token_123";

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareImpl<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareImpl { service: Rc::new(service) })
    }
}

pub struct AuthMiddlewareImpl<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION).cloned();

        let svc = self.service.clone();

        Box::pin(async move {
            match auth_header.and_then(|v| v.to_str().ok().map(|s| s.to_owned())) {
                Some(header_value) if header_value == format!("Bearer {}", API_TOKEN) => {
                    svc.call(req).await
                }
                _ => {
                    Err(actix_web::error::ErrorUnauthorized(
                        "Unauthorized. Set Bearer token in Authorization header.",
                    ))
                }
            }
        })
    }
}
