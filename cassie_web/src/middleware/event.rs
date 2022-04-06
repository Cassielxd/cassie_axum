use axum::{body::Body, http::Request, response::Response};
use cassie_domain::request::RequestModel;
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::Service;

use crate::{
    cici_casbin::is_white_list_api, observe::event::CassieEvent, service::fire_event,
    APPLICATION_CONTEXT,
};
//日志处理核心拦截类
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

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        /*获取method path */
        let action = request.method().clone().to_string();
        let path = request.uri().clone().to_string();
        let creator_name = if !is_white_list_api(path.clone()) {
            let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
            Some(request_model.username.clone())
        } else {
            None
        };
        //调用service
        let future = self.inner.call(request);
        Box::pin(async move {
            //获取时间
            let start = Instant::now();
            //拿到返回值
            let response: Response = future.await?;
            //判断请求返回是不是成功
            let status = if response.status().is_success() {
                Some(1)
            } else {
                Some(0)
            };
            //构建操作日志event对象
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
            //发布事件
            fire_event(event).await;
            Ok(response)
        })
    }
}
