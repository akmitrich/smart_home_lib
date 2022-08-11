use smart_home::smart_device::Device;

#[tokio::main]
async fn main() {
    let c = home_client::HomeClient::new("127.0.0.1:4083").await.unwrap();
    for room in c.get_room_list().await.iter() {
        for device in c.get_device_list(room).await.iter() {
            let device = c.get_device(room, device).await.unwrap();
            println!("In room '{room}' we have {:?}", device);
        }
    }

    loop {
        if let Device::Thermometer(thermometer) = c.get_device("R", "T").await.unwrap() {
            println!("Current temperature is {}", thermometer.get_temperature());
        } else {
            println!("Thermometer is lost...")
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}