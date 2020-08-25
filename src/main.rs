mod server;
mod test;

fn main() {
    env_logger::init();
    if test::run() {
        server::run();
    }
}
