use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;

#[async_std::main]
async fn main() {
    for _ in 1..100_000 {
        UnixStream::connect("/run/user/1000/sway-ipc.1000.871.sock")
            .await
            .unwrap()
            .write(&[])
            .await
            .unwrap();
    }
}
