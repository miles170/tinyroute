use std::future::Future;

pub use smol::Timer;
pub use smol::io::{
    AsyncRead,
    AsyncWrite,
    AsyncWriteExt,
    AsyncReadExt
};

mod tcp;
mod uds;

pub use tcp::{TcpListener, TcpClient};
pub use uds::{UdsListener, UdsClient};

pub async fn sleep(time: std::time::Duration) {
    Timer::after(time).await;
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) {
    smol::spawn(future).detach()
}
