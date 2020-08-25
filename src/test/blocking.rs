use super::{MSG, SOCKET_PATH};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub fn run() {
    UnixStream::connect(SOCKET_PATH)
        .unwrap()
        .write(MSG)
        .unwrap();
}
