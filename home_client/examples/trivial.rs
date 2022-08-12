#[tokio::main]
async fn main() {
    let c = home_client::HomeClient::new("127.0.0.1:4083")
        .await
        .unwrap();
    println!("Room list: {:?}", c.get_room_list().await);
}
