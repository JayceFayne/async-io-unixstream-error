use super::{MSG, SOCKET_PATH};
use async_io::Async;
use futures_lite::{future, AsyncWriteExt};
use std::os::unix::net::UnixStream;

pub fn run() {
    future::block_on(async {
        connection().await.unwrap().write(MSG).await.unwrap();
    });
}

async fn connection() -> std::io::Result<Async<UnixStream>> {
    use async_io::Timer;
    use std::io::ErrorKind;
    use std::time::Duration;

    loop {
        match Async::<UnixStream>::connect(SOCKET_PATH).await {
            Err(err) if err.kind() == ErrorKind::NotConnected => {
                Timer::after(Duration::from_millis(100)).await;
            }
            Err(err) => return Err(err),
            Ok(conn) => return Ok(conn),
        }
    }
}
