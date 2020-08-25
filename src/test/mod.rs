mod async_io;
mod async_std;
mod blocking;

pub const SOCKET_PATH: &str = "/tmp/greet.sock";
pub const MSG: &[u8; 2] = &[104, 105];

use log::trace;
use std::thread::spawn;

pub fn run() -> bool {
    let mut args = std::env::args();
    args.next();
    let run = match args.next().as_deref() {
        Some("blocking") => || blocking::run(),
        Some("async-std") => || async_std::run(),
        Some("async-io") => || async_io::run(),
        Some(_) | None => return false,
    };
    spawn(move || {
        for i in 0..10000 {
            trace!("send msg #{}", i);
            run()
        }
    });
    return true;
}
