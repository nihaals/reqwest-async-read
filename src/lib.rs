use futures::TryStreamExt;
use tokio::io::AsyncRead;
use tokio_util::compat::FuturesAsyncReadCompatExt;

/// Extension trait that allows converting [`reqwest::Response`] into a [`tokio::io::AsyncRead`].
pub trait ReqwestResponseAsyncReadExt {
    fn into_async_read(self) -> impl AsyncRead;
}

impl ReqwestResponseAsyncReadExt for reqwest::Response {
    fn into_async_read(self) -> impl AsyncRead {
        self.bytes_stream()
            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
            .into_async_read()
            .compat()
    }
}
