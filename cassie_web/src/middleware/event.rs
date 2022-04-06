use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};
#[derive(Clone)]
pub struct EventMiddleware<S> {
    pub inner: S,
}
impl<S> Service<Request<Body>> for EventMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}
