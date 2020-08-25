use crate::test::SOCKET_PATH;
use log::info;
use std::fs;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn run() {
    let path = Path::new(SOCKET_PATH);
    if fs::metadata(&path).is_ok() {
        fs::remove_file(&path).unwrap();
    }
    let mut buf = [0_u8, 2];
    for (i, stream) in UnixListener::bind(&path).unwrap().incoming().enumerate() {
        thread::sleep(Duration::new(1, 0));
        stream.unwrap().read(&mut buf).unwrap();
        info!("received msg #{}", i);
    }
}
