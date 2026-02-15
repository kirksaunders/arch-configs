use std::{
    pin::Pin,
    task::{Context, Poll, ready},
};

use tokio::{
    io::AsyncWrite,
    sync::{RwLock, RwLockWriteGuard},
};
use tokio_util::sync::ReusableBoxFuture;

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
    observer: &'a RwLock<T>,
    lock_future: ReusableBoxFuture<'a, RwLockWriteGuard<'a, T>>,
}

impl<'a, T: Send + Sync> AsyncWriteInterceptor<'a, T> {
    pub fn new(observer: &'a RwLock<T>) -> Self {
        Self {
            observer,
            // Go ahead and create the first future
            lock_future: ReusableBoxFuture::new(observer.write()),
        }
    }

    fn acquire_lock(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<RwLockWriteGuard<'a, T>> {
        // Acquire the write lock before proceeding. We're using tokio's async RwLock, so we have
        // to delegate to its poll method.
        let observer = ready!(self.lock_future.poll(cx));
        // At this point, we've successfully acquired the lock. We can call the observer
        // and also create the next lock future
        let new_fut = self.observer.write();
        self.lock_future.set(new_fut);
        Poll::Ready(observer)
    }
}

impl<'a, T: AsyncWriteObserver + Send + Sync> AsyncWrite for AsyncWriteInterceptor<'a, T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut observer = ready!(self.acquire_lock(cx));
        observer.write(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let mut observer = ready!(self.acquire_lock(cx));
        observer.flush();
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        let mut observer = ready!(self.acquire_lock(cx));
        observer.shutdown();
        Poll::Ready(Ok(()))
    }
}
