use std::net::ToSocketAddrs;

use stp::{client::{StpClient, RequestResult}, error::ConnectResult};

pub struct HomeClient(StpClient);

impl HomeClient {
    pub fn new<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let stp_client = StpClient::connect(addr)?;
        Ok(Self(stp_client))
    }

    pub fn switch(&mut self, room_name: &str, device_name: &str, on: bool) -> RequestResult {
        let on_off = if on {"on"} else {"off"};
        let r = format!("switch///{}///{}///{}", room_name, device_name, on_off);
        self.0.send_request(r)
    }

    pub fn set_voltage(&mut self, room_name: &str, device_name: &str, v: f64) -> RequestResult {
        let r = format!("set voltage///{}///{}///{}", room_name, device_name, v);
        self.0.send_request(r)
    }

    pub fn set_current(&mut self, room_name: &str, device_name: &str, c: f64) -> RequestResult {
        let r = format!("set current///{}///{}///{}", room_name, device_name, c);
        self.0.send_request(r)
    }

    pub fn get_power(&mut self, room_name: &str, device_name: &str) -> RequestResult {
        let r = format!("get power///{}///{}", room_name, device_name);
        self.0.send_request(r)
    }

    pub fn get_report(&mut self, room_name: &str, device_name: &str) -> RequestResult {
        let r = format!("get report///{}///{}", room_name, device_name);
        self.0.send_request(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        let response = c.set_current("No room", "No device", 0.).unwrap();
        assert!(response.starts_with("Syntax error"));
    }

    #[test]
    fn on_off() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        let response = c.switch("R", "S", true).unwrap();
        assert_eq!("Socket S is now on", response);
        let response = c.switch("R", "S", false).unwrap();
        assert_eq!("Socket S is now off", response);
    }

    #[test]
    fn set_voltage() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        let response = c.set_voltage("R", "S", 44.).unwrap();
        assert_eq!("Set voltage 44 for socket S", response);
    }

    #[test]
    fn set_current() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        let response = c.set_current("R", "S", 1.).unwrap();
        assert_eq!("Set current 1 for socket S", response);
    }

    #[test]
    fn get_power() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        c.set_voltage("R", "S", 44.).unwrap();
        c.set_current("R", "S", 1.).unwrap();
        let response = c.get_power("R", "S").unwrap();
        assert_eq!("Power for socket S is 44", response);
    }

    #[test]
    fn get_report() {
        let mut c = HomeClient::new("127.0.0.1:4083").unwrap();
        c.set_voltage("R", "S", 44.).unwrap();
        c.set_current("R", "S", 1.).unwrap();
        c.switch("R", "S", true).unwrap();
        let response = c.get_report("R", "S").unwrap();
        assert_eq!("is on; current power is 44", response);
    }
}
