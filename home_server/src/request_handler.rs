#![allow(unused, dead_code)]

use smart_home::{
    home::Home,
    smart_device::{Device, DeviceInfo, Socket},
};
use std::{str::Split, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Request<'a>(Split<'a, &'a str>);

pub struct Handler(Arc<RwLock<Home>>);

impl<'a> Request<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self(raw.split("///"))
    }

    fn proceed(&mut self) -> &'a str {
        self.0.next().unwrap_or("").trim()
    }
}

impl Handler {
    pub fn new(h: Arc<RwLock<Home>>) -> Self {
        Self(h)
    }

    pub async fn respond<'a>(&'a mut self, r: &'a mut Request<'_>) -> String {
        let cmd = r.proceed();
        match cmd {
            "room list" => self.room_list(r).await,
            "device list" => self.device_list(r).await,
            "get device" => self.get_device(r).await,
            "update device" => self.update_device(r).await,
            _ => String::from("Bad command"),
        }
    }

    async fn room_list(&self, r: &mut Request<'_>) -> String {
        let mut result = vec![String::from("Ok")];
        let home = self.0.read().await;
        for room in home.room_names_list() {
            result.push(room.clone());
        }
        result.join("///")
    }

    async fn device_list(&self, r: &mut Request<'_>) -> String {
        let mut result = vec![];
        let home = self.0.read().await;
        let room_name = r.proceed();
        if let Some(room) = home.get_room_by_name(room_name) {
            result.push("Ok".into());
            for d in room.device_names_list() {
                result.push(d.clone());
            }
        } else {
            result.push("Err".into());
            result.push(format!("Room '{room_name}' not found."));
        }
        result.join("///")
    }

    async fn get_device(&self, r: &mut Request<'_>) -> String {
        let mut result = vec![];
        let home = self.0.read().await;
        let room = r.proceed();
        let device = r.proceed();
        match home.get_device_by_path(room, device) {
            Some(device) => {
                result.push("Ok".into());
                result.append(&mut device.device_info());
            }
            None => {
                result.push("Err".into());
                result.push(format!("Device '{}' not found in room '{}'", device, room));
            }
        }
        result.join("///")
    }

    async fn update_device(&self, r: &mut Request<'_>) -> String {
        let mut home = self.0.write().await;
        let room_name = r.proceed();
        let device_name = r.proceed();
        let mut result = vec![];
        if let Some(device) = home.get_device_by_path_mut(room_name, device_name) {
            result.push("Ok".into());
            update_from_stp_request(device, r);
        } else {
            result.push("Err".into());
            result.push(format!(
                "Device '{device_name}' not found in room '{room_name}'."
            ));
        }
        result.join("///")
    }
}

fn update_from_stp_request(device: &mut Device, req: &mut Request) {
    match device {
        Device::Socket(socket) if req.proceed() == "socket" => {
            let on = "on" == req.proceed();
            if let Ok(current) = req.proceed().parse::<f64>() {
                if let Ok(voltage) = req.proceed().parse::<f64>() {
                    socket.switch(on);
                    socket.set_current(current);
                    socket.set_voltage(voltage);
                }
            }
        }
        Device::Thermometer(thermometer) if req.proceed() == "thermometer" => {
            if let Ok(temperature) = req.proceed().parse::<f64>() {
                thermometer.set_temperature(temperature);
            }
        }
        _ => todo!(),
    }
}
