use std::{
    future::Future,
    pin::Pin,
    task::{ready, Poll},
};

use tokio::{
    io::AsyncWrite,
    sync::{Mutex, MutexGuard},
};

/// Observer for AsyncWrite operations, which are exposed through `AsyncWriteInterceptor`.
#[allow(unused)]
pub trait AsyncWriteObserver {
    fn write(&mut self, data: &[u8]) {}

    fn flush(&mut self) {}

    fn shutdown(&mut self) {}
}

/// Struct that implements AsyncWrite and allows a (sync) observer to handle write operations.
/// As an example use case, you could write an observer to buffer writes in memory, then pass
/// the interceptor to any function accepts impl AsyncWrite.
pub struct AsyncWriteInterceptor<'a, T> {
    observer: &'a Mutex<T>,
    lock_future: Option<Pin<Box<dyn Future<Output = MutexGuard<'a, T>> + Send + 'a>>>,
}

impl<'a, T> AsyncWriteInterceptor<'a, T> {
    pub fn new(observer: &'a Mutex<T>) -> Self {
        Self {
            observer,
            lock_future: None,
        }
    }
}

impl<'a, T: AsyncWriteObserver + Send> AsyncWrite for AsyncWriteInterceptor<'a, T> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        // Acquire the mutex lock before proceeding. We're using tokio's async
        // mutex, so we have to delegate to its poll method.
        if let None = this.lock_future {
            let fut = Box::pin(this.observer.lock());
            this.lock_future = Some(fut);
        }
        let mut observer = ready!(this.lock_future.as_mut().unwrap().as_mut().poll(cx));
        // At this point, we've successfully acquired the lock
        this.lock_future = None;
        observer.write(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
