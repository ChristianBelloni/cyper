use std::{future::Future, pin::Pin, task::Poll};

use compio::bytes::Bytes;
use tonic::client::GrpcService;
use url::Url;

use crate::{Client, IntoUrl};

/// Client usable with [`tonic`] as a [`GrpcService`]
pub struct TonicClient {
    url: Url,
    client: Client,
}

impl TonicClient {
    pub fn new(url: impl IntoUrl) -> Self {
        Self::with_client(url, Client::new())
    }

    pub fn with_client(url: impl IntoUrl, client: Client) -> Self {
        Self {
            url: url.into_url().unwrap(),
            client,
        }
    }
}

impl GrpcService<tonic::body::Body> for TonicClient {
    type ResponseBody = hyper::body::Incoming;

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
        let client = self.client.clone();
        let url = self.url.clone();
        Box::pin(async move {
            let (parts, body) = request.into_parts();
            let method = parts.method;
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
