#![allow(unused, dead_code)]

use std::{
    str::Split,
    sync::Arc,
};
use tokio::sync::RwLock;
use smart_home::{
    home::Home,
    smart_device::{Device, DeviceInfo, Socket},
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
            "room list" => "self.room_list(r)".into(),
            "device_list" => "self.device_list(r)".into(),
            "get device" => "self.get_device(r)".into(),
            "set device" => "self.set_device(r)".into(),
            _ => String::from("Bad command"),
        }
    }
}
