use std::collections::{HashMap, hash_map::{Keys, Values}};
use crate::smart_device::Device;

#[allow(dead_code, unused)]
pub struct Room {
    devices: HashMap<String, Device>,
}

#[allow(dead_code, unused)]
impl Room {
    pub fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }

    pub fn device_names_list(&self) -> Keys<String, Device> {
        todo!()
    }

    pub fn device_list(&self) -> Values<String, Device> {
        todo!()
    }

    pub fn add_device(&mut self, unique_name: &str, device: Device) -> Option<&Device> {
        todo!()
    }

    pub fn remove_device(&mut self, unique_name: &str) -> Option<Device>
    {
        todo!()
    }

    pub fn get_device_by_name(&self, device_name: &str) -> Option<&Device> {
        todo!()
    }
}
