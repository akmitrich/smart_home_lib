#![allow(unused, dead_code)]
use tokio::net::ToSocketAddrs;

use stp::{
    client::StpClient,
    error::ConnectResult,
};

pub struct HomeClient(StpClient);

impl HomeClient {
    pub async fn new<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stp_client = StpClient::connect(addr).await?;
        Ok(Self(stp_client))
    }

    pub async fn get_socket(&self, room_name: &str, device_name: &str) {
        self.0.send_request("Hello server!").await.unwrap();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         let response = c.set_current("No room", "No device", 0.).unwrap();
//         assert!(response.starts_with("Syntax error"));
//     }

//     #[test]
//     fn on_off() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         let response = c.switch("R", "S", true).unwrap();
//         assert_eq!("Socket S is now on", response);
//         let response = c.switch("R", "S", false).unwrap();
//         assert_eq!("Socket S is now off", response);
//     }

//     #[test]
//     fn set_voltage() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         let response = c.set_voltage("R", "S", 44.).unwrap();
//         assert_eq!("Set voltage 44 for socket S", response);
//     }

//     #[test]
//     fn set_current() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         let response = c.set_current("R", "S", 1.).unwrap();
//         assert_eq!("Set current 1 for socket S", response);
//     }

//     #[test]
//     fn get_power() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         c.set_voltage("R", "S", 44.).unwrap();
//         c.set_current("R", "S", 1.).unwrap();
//         let response = c.get_power("R", "S").unwrap();
//         assert_eq!("Power for socket S is 44", response);
//     }

//     #[test]
//     fn get_report() {
//         let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
//         c.set_voltage("R", "S", 44.).unwrap();
//         c.set_current("R", "S", 1.).unwrap();
//         c.switch("R", "S", true).unwrap();
//         let response = c.get_report("R", "S").unwrap();
//         assert_eq!("is on; current power is 44", response);
//     }
// }
