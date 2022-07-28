use std::error::Error;
use stp::client::StpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = StpClient::connect("127.0.0.1:5555").await?;
    let response = client.send_request("Hello, server").await?;
    assert_eq!(response, "Hello, client");
    Ok(())
}
