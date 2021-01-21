use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};
use tokio::sync::oneshot;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DropBearRecvError {
  SenderDropped,
}

/// A future which resolves to the contents of a DropBear instance after it is dropped
pub struct DropBearHandle<T>(oneshot::Receiver<T>);

impl<T> Future for DropBearHandle<T> {
  type Output = Result<T, DropBearRecvError>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    match oneshot::Receiver::poll(Pin::new(&mut self.0), cx) {
      Poll::Pending => Poll::Pending,
      Poll::Ready(Ok(x)) => Poll::Ready(Ok(x)),
      Poll::Ready(Err(_e)) => Poll::Ready(Err(DropBearRecvError::SenderDropped)),
    }
  }
}

/// A container which fires a notification to a future when it is dropped
pub struct DropBear<T> {
  inner: Option<(T, oneshot::Sender<T>)>,
}

impl<T> DropBear<T> {
  pub fn new(item: T) -> (Self, DropBearHandle<T>) {
    let (send, recv) = oneshot::channel();
    (
      Self {
        inner: Some((item, send)),
      },
      DropBearHandle(recv),
    )
  }

  /// Note that `into_inner` will destroy the sender, firing an error to the receiver
  pub fn into_inner(mut self) -> T {
    match std::mem::replace(&mut self.inner, None) {
      None => unreachable!("into_inner called multiple times (?) or after drop?"),
      Some((val, _sender)) => val,
    }
  }
}

impl<T> Drop for DropBear<T> {
  fn drop(&mut self) {
    match std::mem::replace(&mut self.inner, None) {
      None => {
        // into_inner extracted the value
      }
      Some((val, sender)) => match sender.send(val) {
        Ok(()) => {}
        Err(_val) => {
          // Drop the value here, as the receiver handle was dropped before we could send to it
        }
      },
    }
  }
}

impl<T> std::ops::Deref for DropBear<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match &self.inner {
      None => unreachable!("DropBear dereferenced after drop?"),
      Some((x, _sender)) => x,
    }
  }
}

impl<T> std::ops::DerefMut for DropBear<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match &mut self.inner {
      None => unreachable!("DropBear dereferenced after drop?"),
      Some((x, _sender)) => x,
    }
  }
}

impl<T> std::convert::AsRef<T> for DropBear<T> {
  fn as_ref(&self) -> &T {
    &*self
  }
}

impl<T> std::convert::AsMut<T> for DropBear<T> {
  fn as_mut(&mut self) -> &mut T {
    &mut *self
  }
}

impl<T: std::fmt::Debug> std::fmt::Debug for DropBear<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(self.as_ref(), f)
  }
}

impl<T: std::fmt::Display> std::fmt::Display for DropBear<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.as_ref(), f)
  }
}

impl<T: std::cmp::PartialEq> std::cmp::PartialEq for DropBear<T> {
  fn eq(&self, other: &Self) -> bool {
    self.as_ref().eq(other.as_ref())
  }
}

impl<T: std::cmp::Eq + std::cmp::PartialEq> std::cmp::Eq for DropBear<T> {}

impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for DropBear<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.as_ref().partial_cmp(other.as_ref())
  }
}

impl<T: std::cmp::Ord + std::cmp::PartialOrd> std::cmp::Ord for DropBear<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.as_ref().cmp(other.as_ref())
  }
}
