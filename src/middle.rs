use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{Service, Transform};
use futures_util::future::LocalBoxFuture;

pub struct CacheControlMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CacheControlMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CacheControlMiddlewareService<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move { Ok(CacheControlMiddlewareService { service }) })
    }
}

pub struct CacheControlMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CacheControlMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if res.request().path().starts_with("/static") {
                res.headers_mut().insert(
                    actix_web::http::header::CACHE_CONTROL,
                    actix_web::http::header::HeaderValue::from_static("public, max-age=3600"),
                );
            }
            Ok(res)
        })
    }
}