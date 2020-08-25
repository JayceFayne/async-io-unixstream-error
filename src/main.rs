use async_io::Async;
use futures_lite::future;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    for _ in 1..100 {
        let handle = thread::spawn(|| {
            future::block_on(async {
                Async::<UnixStream>::connect("/run/user/1000/sway-ipc.1000.871.sock")
                    .await?
                    .write_with_mut(|io| io.write(&[]))
                    .await
            })
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(error) = handle.join().unwrap() {
            println!("{:?}", error)
        }
    }
}
