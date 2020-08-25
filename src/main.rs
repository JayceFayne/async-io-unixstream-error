use std::io::Write;
use std::os::unix::net::UnixStream;
use std::thread::spawn;

fn main() {
    let mut guard = Vec::new();
    for _ in 1..30000 {
        guard.push(spawn(|| {
            UnixStream::connect("/run/user/1000/sway-ipc.1000.871.sock")
                .unwrap()
                .write(&[])
                .unwrap()
        }));
    }
}

/* above ~30k it panics with:
thread '<unnamed>' panicked at 'failed to set up alternative stack guard page', library/std/src/sys/unix/stack_overflow.rs:156:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'thread '<unnamed>mainfatal runtime error: ' panicked at '' panicked at 'failed to initiate panic, error failed to set up alternative stack guard pagefailed to spawn thread: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }5', ',
library/std/src/sys/unix/stack_overflow.rs/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs::156619::1329
*/
