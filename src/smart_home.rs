use std::collections::{HashMap, hash_map::{Keys, Values}};
use crate::smart_room::Room;
use crate::smart_device::Device;
use crate::FetchResult;

#[allow(dead_code, unused)]
pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

#[allow(dead_code, unused)]
impl Home {
    pub fn new(name: &str) -> Self {
        Home {
            name: String::from(name),
            rooms: HashMap::new(),
        }
    }

    pub fn room_list(&self) -> Values<String, Room> {
        todo!()
    }

    pub fn room_names_list(&self) -> Keys<String, Room> {
        todo!()
    }

    pub fn add_room(&mut self, unique_name: &str) -> Option<Room> {
        todo!()
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room>
    {
        todo!()
    }

    pub fn get_room_by_name(&self, room_name: &str) -> Option<Room> {
        todo!()
    }

    pub fn device_names_list(&self) -> Vec<&str> {
        todo!()
    }

    pub fn add_device(&mut self, room_name: &str, unique_name: &str, device: Device) -> Option<Device> {
        todo!()
    }

    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Option<Device> {
        todo!()
    }

    pub fn get_device_by_path(&self, room_name: &str, device_name: &str) -> Option<Device> {
        todo!()
    }

    pub fn report(&self) -> String {
        todo!()
    }
}
