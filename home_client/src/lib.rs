#![allow(unused, dead_code)]
use std::vec;

use tokio::net::ToSocketAddrs;

use stp::{
    client::{StpClient, RequestResult, RequestError},
    error::ConnectResult,
};

use smart_home::smart_device::{Device, Socket, Thermometer, DeviceInfo};

pub struct HomeClient(StpClient);

impl HomeClient {
    pub async fn new<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stp_client = StpClient::connect(addr).await?;
        Ok(Self(stp_client))
    }

    pub async fn get_room_list(&self) -> Vec<String> {
        let mut result = vec![];
        let response = self.0.send_request("room list").await.unwrap_or_default();
        let mut response = response.split("///");
        if let Some(code) = response.next() {
            if code == "Ok" {
                for room in response {
                    result.push(String::from(room));
                }
            }
        }
        result
    }

    pub async fn get_device_list(&self, room_name: &str) -> Vec<String> {
        let mut result = vec![];
        let response = self.0.send_request(format!("device list///{room_name}")).await.unwrap_or_default();
        let mut response = response.split("///");
        if let Some(code) = response.next() {
            if code == "Ok" {
                for device in response {
                    result.push(String::from(device));
                }
            }
        }
        result
    }

    pub async fn get_device(&self, room_name: &str, device_name: &str) -> Result<Device, Box<dyn std::error::Error>> {
        let response = self.0.send_request(format!("get device///{room_name}///{device_name}")).await?;
        println!("From server: {response}");
        let mut response = response.split("///");
        let code = response.next().unwrap_or_default();
        if code == "Ok" {
            return Ok(from_stp_response(&mut response));
        }
        Ok(Device::Unknown)
    }

    pub async fn update_device(&self, room_name: &str, device_name: &str, device: Device) -> RequestResult {
        let info = device.device_info().join("///");
        let response = self.0.send_request(format!("update device///{room_name}///{device_name}///{info}")).await?;
        println!("From server: {response}");
        Ok("Ok".into())
    }

}

fn from_stp_response<'a>(response: &'a mut impl Iterator<Item = &'a str>) -> Device {
    let device = response.next().unwrap_or_default();
    match device {
        "socket" => {
            let on_off = response.next().unwrap_or("off");
            let on = on_off == "on";
            let current: f64 = response.next().unwrap_or_default().parse().unwrap_or_default();
            if let Ok(voltage) = response.next().unwrap_or_default().parse::<f64>() {
                Device::Socket(Socket::new(voltage, current, on))
            } else {
                Device::Unknown
            }
        }
        "thermometer" => {
            if let Ok(temperature) = response.next().unwrap_or_default().parse::<f64>() {
                Device::Thermometer(Thermometer::new(temperature))
            } else {
                Device::Unknown
            }
        }
        _ => Device::Unknown,
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
