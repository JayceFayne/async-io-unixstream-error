use swayipc::Connection;

#[async_std::main]
async fn main() {
    for _ in 0..100_000 {
        Connection::new()
            .await
            .unwrap()
            .get_bar_ids()
            .await
            .unwrap();
    }
}
