use axum::{body::Body, http::Request, response::Response};
use cassie_domain::request::RequestModel;
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

use crate::{cici_casbin::is_white_list_api, observe::event::CassieEvent, APPLICATION_CONTEXT};
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
        /*获取method path */
        let action = request.method().clone().to_string();
        let path = request.uri().clone().to_string();

        let creator_name = if !is_white_list_api(path.clone()) {
            let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
            Some(request_model.username.clone())
        } else {
            None
        };

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            let mut event = CassieEvent::LogOperation {
                operation: None,
                request_uri: Some(path.clone()),
                ip: None,
                creator_name: creator_name,
                request_params: None,
                request_method: None,
                request_time: None,
                status: None,
            };

            Ok(response)
        })
    }
}
