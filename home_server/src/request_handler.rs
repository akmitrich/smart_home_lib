#![allow(unused, dead_code)]

use std::{
    str::Split,
    sync::{Arc, RwLock},
};

use smart_home::{
    home::Home,
    smart_device::{Device, Socket, ReportState},
};

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

    pub fn respond(&mut self, r: &mut Request) -> String {
        let cmd = r.proceed();
        match cmd {
            "set voltage" => self.set_voltage(r),
            "set current" => self.set_current(r),
            "get power" => self.get_power(r),
            "get report" => self.get_report(r),
            _ => String::from("Bad command"),
        }
    }

    fn set_voltage(&mut self, r: &mut Request) -> String {
        let room = r.proceed();
        let device = r.proceed();
        if let Ok(mut home) = self.0.write() {
            if let Some(Device::Socket(s)) = home.get_device_by_path_mut(room, device) {
                if let Ok(v) = r.proceed().parse::<f64>() {
                    s.set_voltage(v);
                    return format!("Set voltage {} for socket {}", v, device);
                }
            }
        }
        format!("Syntax error in request: {:?}", r)
    }

    fn set_current(&self, r: &mut Request) -> String {
        let room = r.proceed();
        let device = r.proceed();
        if let Ok(mut home) = self.0.write() {
            if let Some(Device::Socket(s)) = home.get_device_by_path_mut(room, device) {
                if let Ok(c) = r.proceed().parse::<f64>() {
                    s.set_current(c);
                    return format!("Set current {} for socket {}", c, device);
                }
            }
        }
        format!("Syntax error in request: {:?}", r)
    }

    fn get_power(&self, r: &mut Request) -> String {
        let room = r.proceed();
        let device = r.proceed();
        if let Ok(home) = self.0.read() {
            if let Some(Device::Socket(s)) = home.get_device_by_path(room, device) {
                if let Ok(c) = r.proceed().parse::<f64>() {
                    let p = s.get_current_power();
                    return format!("Power for socket {} is {}", device, p);
                }
            }
        }
        format!("Syntax error in request: {:?}", r)
    }

    pub fn get_report(&self, r: &mut Request) -> String {
        let room = r.proceed();
        let device = r.proceed();
        if let Ok(home) = self.0.read() {
            if let Some(Device::Socket(s)) = home.get_device_by_path(room, device) {
                return s.report_state();
            }
        }
        format!("Syntax error in request: {:?}", r)
    }
}
