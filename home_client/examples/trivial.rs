#[tokio::main]
async fn main() {
    let c = home_client::HomeClient::new("127.0.0.1:4083").await.unwrap();
    c.get_socket("R", "S").await;
}