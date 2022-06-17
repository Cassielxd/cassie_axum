use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;
use crate::middleware::clear_local;

//日志处理核心拦截类
#[derive(Clone)]
pub struct ContextMiddleware<S> {
    pub inner: S,
}
impl<S> Service<Request<Body>> for ContextMiddleware<S>
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
        //调用service
        let future = self.inner.call(request);
        Box::pin(async move {
            //拿到返回值
            let response: Response = future.await?;
            //清除线程变量 防止环境污染
            clear_local();
            Ok(response)
        })
    }
}
