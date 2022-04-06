use axum::{body::Body, http::Request, response::Response};
use cassie_domain::request::RequestModel;
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::Service;

use crate::{
    cici_casbin::is_white_list_api, observe::event::CassieEvent,
    service::event_service::fire_event, APPLICATION_CONTEXT,
};
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
        let body = request.body().clone();

        let creator_name = if !is_white_list_api(path.clone()) {
            let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
            Some(request_model.username.clone())
        } else {
            None
        };
        //pharos.notify(event).await;
        let future = self.inner.call(request);
        Box::pin(async move {
            let start = Instant::now();
            let response: Response = future.await?;
            let status = if response.status().is_success() {
                Some(1)
            } else {
                Some(0)
            };
            let event = CassieEvent::LogOperation {
                operation: Some(action.clone()),
                request_uri: Some(path.clone()),
                ip: None,
                creator_name,
                request_params: None,
                request_method: Some(action.clone()),
                request_time: Some(start.elapsed().as_millis().to_string()),
                status,
            };
            fire_event(event).await;
            Ok(response)
        })
    }
}
