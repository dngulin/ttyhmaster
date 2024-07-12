use axum::http::{Request, Response, Uri};
use std::task::{Context, Poll};
use tower::Service;

#[derive(Debug, Clone)]
pub struct ReRoute<S> {
    inner: S,
    mappings: Vec<(Uri, Uri)>,
}

impl<S> ReRoute<S> {
    pub fn wrap(inner: S, mappings: Vec<(Uri, Uri)>) -> Self {
        Self { inner, mappings }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ReRoute<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let uri = req.uri_mut();

        for (from, to) in &self.mappings {
            if uri == from {
                tracing::debug!("Reroute `{from}` to `{to}`");
                *uri = to.clone();
                break;
            }
        }

        self.inner.call(req)
    }
}
