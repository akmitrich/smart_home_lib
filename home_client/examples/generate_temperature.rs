use smart_home::smart_device::{Device, Thermometer};

#[tokio::main]
async fn main() {
    let c = home_client::HomeClient::new("127.0.0.1:4083").await.unwrap();
    c.update_device("R", "T", Device::Thermometer(Thermometer::new(12.1))).await.unwrap();
    for _ in 0..10 {
        if let Device::Thermometer(t) = c.get_device("R", "T").await.unwrap() {
            let temperature = t.get_temperature();
            c.update_device("R", "T", Device::Thermometer(Thermometer::new(temperature + 0.1))).await.unwrap();
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}