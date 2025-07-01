use std::{future::Future, pin::Pin, task::Poll};

use axum::body::Bytes;
use tonic::client::GrpcService;
use url::Url;

use crate::Client;

impl GrpcService<tonic::body::Body> for Client {
    type ResponseBody = Bytes;

    type Error = crate::Error;

    type Future =
        Pin<Box<dyn Future<Output = Result<http::Response<Self::ResponseBody>, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: hyper::Request<tonic::body::Body>) -> Self::Future {
        let client = self.clone();
        Box::pin(async move {
            let (parts, body) = request.into_parts();

            let method = parts.method;
            let url = parts.uri.to_string();
            let headers = parts.headers;
            let version = parts.version;

            client
                .request(method, url)?
                .headers(headers)
                .version(version)
                .body(body)
                .send()
                .await?;
            todo!()
        })
    }
}
