use crate::middleware::auth::get_local;
use crate::{cici_casbin::is_white_list_api, observe::event::CassieEvent, service::fire_event};
use axum::body::Bytes;
use axum::{body::Body, http::Request, response::Response};
use cassie_domain::dto::sys_log::SysLogOperationDto;
use futures::future::BoxFuture;
use log::info;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::Service;

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
            let request_model = get_local().unwrap();
            Some(request_model.username().clone())
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
            let mut operation = SysLogOperationDto::default();
            operation.set_operation(Some(action.clone()));
            operation.set_request_uri(Some(path.clone()));
            operation.set_creator_name(creator_name);
            operation.set_status(status);
            operation.set_request_time(Some(start.elapsed().as_millis().to_string()));
            operation.set_request_method(Some(action.clone()));
            //发布事件
            fire_event(CassieEvent::LogOperation(operation)).await;
            //APPLICATION_CONTEXT.set_local::<RequestModel>(|| None);
            Ok(response)
        })
    }
}
