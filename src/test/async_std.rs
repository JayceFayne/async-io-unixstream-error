use super::{MSG, SOCKET_PATH};
use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use futures_lite::future;

pub fn run() {
    future::block_on(async {
        UnixStream::connect(SOCKET_PATH)
            .await
            .unwrap()
            .write(MSG)
            .await
            .unwrap()
    });
}
