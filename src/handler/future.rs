//! Handler future types.

use futures_util::future::Map;
use pin_project_lite::pin_project;
use std::{convert::Infallible, future::Future, pin::Pin, task::Context};
use tower::util::Oneshot;
use tower_service::Service;

opaque_future! {
    /// The response future for [`IntoService`](super::IntoService).
    pub type IntoServiceFuture<Response, F> =
        Map<
            F,
            fn(Response) -> Result<Response, Infallible>,
        >;
}

pin_project! {
    /// The response future for [`Layered`](super::Layered).
    pub struct LayeredFuture<Request, Response, S>
    where
        S: Service<Request>,
    {
        #[pin]
        inner: Map<Oneshot<S, Request>, fn(Result<S::Response, S::Error>) -> Response>,
    }
}

impl<Request, Response, S> LayeredFuture<Request, Response, S>
where
    S: Service<Request>,
{
    pub(super) fn new(
        inner: Map<Oneshot<S, Request>, fn(Result<S::Response, S::Error>) -> Response>,
    ) -> Self {
        Self { inner }
    }
}

impl<Request, Response, S> Future for LayeredFuture<Request, Response, S>
where
    S: Service<Request>,
{
    type Output = Response;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}
