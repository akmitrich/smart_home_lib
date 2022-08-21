#![allow(unused, dead_code)]
use std::vec;

use tokio::net::ToSocketAddrs;

use stp::{
    client::{RequestError, RequestResult, StpClient},
    error::ConnectResult,
};

use smart_home::smart_device::{Device, DeviceInfo, Socket, Thermometer};

const OK_RESPONSE: &str = "Ok";
const ERR_RESPONSE: &str = "Err";
const SEPARATOR: &str = "///";

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
        let response = self.0.send_request("room list").await.unwrap_or_default();
        let mut response = response.split(SEPARATOR);
        list_from_stp_response(&mut response)
    }

    /// TODO: must return Result<Vec<String>, 'Error'>
    pub async fn get_device_list(&self, room_name: &str) -> Vec<String> {
        let response = self
            .0
            .send_request(format!("device list{SEPARATOR}{room_name}"))
            .await
            .unwrap_or_default();
        let mut response = response.split(SEPARATOR);
        list_from_stp_response(&mut response)
    }

    /// TODO: define new Error for Result
    pub async fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<Device, Box<dyn std::error::Error>> {
        let response = self
            .0
            .send_request(format!(
                "get device{SEPARATOR}{room_name}{SEPARATOR}{device_name}"
            ))
            .await?;
        let mut response = response.split({ SEPARATOR });
        let code = response.next().unwrap_or_default();
        if code == OK_RESPONSE {
            return Ok(device_from_stp_response(&mut response));
        }
        Ok(Device::Unknown)
    }

    pub async fn update_device(
        &self,
        room_name: &str,
        device_name: &str,
        device: Device,
    ) -> RequestResult {
        let info = device.device_info().join(SEPARATOR);
        let response = self
            .0
            .send_request(format!(
                "update device{SEPARATOR}{room_name}{SEPARATOR}{device_name}{SEPARATOR}{info}"
            ))
            .await?;
        println!("From server: {response}");
        Ok(OK_RESPONSE.into())
    }
}

fn device_from_stp_response<'a>(response: &'a mut impl Iterator<Item = &'a str>) -> Device {
    let device = response.next().unwrap_or_default();
    match device {
        "socket" => {
            let on_off = response.next().unwrap_or("off");
            let on = on_off == "on";
            let current: f64 = response
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
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

fn list_from_stp_response<'a>(response: &'a mut impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut result = vec![];
    if let Some(code) = response.next() {
        if code == OK_RESPONSE {
            for item in response {
                result.push(String::from(item));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let mut c = HomeClient::new("127.0.0.1:4083").await.unwrap();
        let response = c.get_device("No room", "No device").await.unwrap();
        assert_eq!(response, Device::Unknown);
    }

    #[tokio::test]
    async fn on_off() {
        let mut c = HomeClient::new("127.0.0.1:4083").await.unwrap();
        let mut response = c.get_device("R", "S").await.unwrap();
        if let Device::Socket(mut socket) = response {
            socket.switch(true);
            let result = c.update_device("R", "S", Device::Socket(socket)).await;
            assert_eq!(OK_RESPONSE, result.unwrap());
            let response = c.get_device("R", "S").await.unwrap();
            if let Device::Socket(socket) = response {
                assert!(socket.is_on());
            } else {
                panic!("Unexpected device after update.")
            }
        } else {
            panic!("Unexpected device comes from server.")
        }
    }

    #[tokio::test]
    async fn set_voltage() {
        let mut c = HomeClient::new("127.0.0.1:4083").await.unwrap();
        let mut response = c.get_device("R", "S").await.unwrap();
        if let Device::Socket(mut socket) = response {
            socket.set_voltage(215.);
            let result = c.update_device("R", "S", Device::Socket(socket)).await;
            assert_eq!(OK_RESPONSE, result.unwrap());
            let response = c.get_device("R", "S").await.unwrap();
            if let Device::Socket(socket) = response {
                assert_eq!(215., socket.get_voltage());
            } else {
                panic!("Unexpected device after update.")
            }
        } else {
            panic!("Unexpected device comes from server.")
        }
    }

    #[tokio::test]
    async fn set_current() {
        let mut c = HomeClient::new("127.0.0.1:4083").await.unwrap();
        let mut response = c.get_device("R", "S").await.unwrap();
        if let Device::Socket(mut socket) = response {
            socket.set_current(5.);
            let result = c.update_device("R", "S", Device::Socket(socket)).await;
            assert_eq!(OK_RESPONSE, result.unwrap());
            let response = c.get_device("R", "S").await.unwrap();
            if let Device::Socket(socket) = response {
                assert_eq!(5., socket.get_current());
            } else {
                panic!("Unexpected device after update.")
            }
        } else {
            panic!("Unexpected device comes from server.")
        }
    }

    #[tokio::test]
    async fn get_power() {
        let mut c = HomeClient::new("127.0.0.1:4083").await.unwrap();
        let mut response = c.get_device("R", "S").await.unwrap();
        if let Device::Socket(mut socket) = response {
            socket.set_current(5.);
            socket.set_voltage(200.);
            let result = c.update_device("R", "S", Device::Socket(socket)).await;
            assert_eq!(OK_RESPONSE, result.unwrap());
            let response = c.get_device("R", "S").await.unwrap();
            if let Device::Socket(socket) = response {
                assert_eq!(1000., socket.get_current_power());
            } else {
                panic!("Unexpected device after update.")
            }
        } else {
            panic!("Unexpected device comes from server.")
        }
    }
}
