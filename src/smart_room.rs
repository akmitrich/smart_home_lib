use std::collections::{HashMap, hash_map::Values};
use crate::smart_device::Device;

#[allow(dead_code, unused)]
pub struct Room {
    unique_name: String,
    devices: HashMap<String, Device>,
}

#[allow(dead_code, unused)]
impl Room {
    pub fn new(unique_name: &str) -> Self {
        Room {
            unique_name: String::from(unique_name),
            devices: HashMap::new(),
        }
    }

    pub fn device_list(&self) -> Values<String, Device> {
        todo!()
    }

    pub fn add_device(&mut self, device: Device, unique_name: &str) {
        todo!()
    }

    pub fn remove_device(&mut self, unique_name: &str) -> Option<Device>
    {
        todo!()
    }
}
