use super::{MSG, SOCKET_PATH};
use async_io::Async;
use futures_lite::{future, AsyncWriteExt};
use std::os::unix::net::UnixStream;

pub fn run() {
    future::block_on(async {
        Async::<UnixStream>::connect(SOCKET_PATH)
            .await
            .unwrap()
            .write(MSG)
            .await
            .unwrap()
    });
}
