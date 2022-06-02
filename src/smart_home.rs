use crate::smart_device::Device;
use crate::smart_room::Room;
use std::collections::{
    hash_map::{Keys, Values},
    HashMap,
};

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

    pub fn room_names_list(&self) -> Keys<String, Room> {
        todo!()
    }

    pub fn room_list(&self) -> Values<String, Room> {
        todo!()
    }

    pub fn add_room(&mut self, unique_name: &str) -> Option<&Room> {
        todo!()
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room> {
        todo!()
    }

    pub fn get_room_by_name(&self, room_name: &str) -> Option<&Room> {
        todo!()
    }

    pub fn device_names_list(&self, room_name: &str) -> Vec<&str> {
        todo!()
    }

    pub fn add_device(
        &mut self,
        room_name: &str,
        unique_name: &str,
        device: Device,
    ) -> Option<&Device> {
        todo!()
    }

    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Option<Device> {
        todo!()
    }

    pub fn get_device_by_path(&self, room_name: &str, device_name: &str) -> Option<&Device> {
        todo!()
    }

    pub fn report(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_home() {
        let home = Home::new("Home");
        assert_eq!("Home", home.name);
        let rooms: Vec<&Room> = home.room_list().collect();
        assert!(rooms.is_empty());
        let room_names: Vec<&String> = home.room_names_list().collect();
        assert!(room_names.is_empty());
    }

    #[test]
    fn test_add_rooms() {
        let mut home = Home::new("Home with rooms");
        assert!(home.add_room("R1").is_some());
        assert!(home.add_room("R2").is_some());
        assert!(home.add_room("R1").is_none());
    }

    #[test]
    fn test_room_list() {
        let mut home = Home::new("Home with rooms");
        home.add_room("R1");
        home.add_room("R2");
        let rooms: Vec<&Room> = home.room_list().collect();
        assert!(rooms.contains(&home.get_room_by_name("R1").unwrap()));
        assert!(rooms.contains(&home.get_room_by_name("R2").unwrap()));
        let room_names: Vec<&String> = home.room_names_list().collect();
        assert!(room_names.contains(&&String::from("R1")));
        assert!(room_names.contains(&&String::from("R2")));
    }

    #[test]
    fn test_remove_room() {
        let mut home = Home::new("Home to delete");
        assert!(home.add_room("R1").is_some());
        assert!(home.add_room("R2").is_some());
        assert!(home.remove_room("R1").is_some());
        assert!(home.remove_room("R2").is_some());
        assert!(home.remove_room("R3").is_none());
        assert!(home.remove_room("R2").is_none());
        assert!(home.remove_room("R1").is_none());
        assert!(home.room_names_list().collect::<Vec<&String>>().is_empty());
        assert!(home.room_list().collect::<Vec<&Room>>().is_empty());
    }

    #[test]
    fn test_add_device() {
        let mut home = Home::new("Home for devices");
        assert!(home.add_room("R1").is_some());
        assert!(home.add_room("R2").is_some());
        assert!(home.add_device("R1", "S1", Device::new_socket()).is_some());
        assert!(home.add_device("R1", "S2", Device::new_socket()).is_some());
        assert!(home
            .add_device("R1", "T", Device::new_thermometer())
            .is_some());
        assert!(home
            .add_device("R1", "S1", Device::new_thermometer())
            .is_none());
        assert!(home.add_device("R2", "S1", Device::new_socket()).is_some());
        assert!(home
            .add_device("R2", "T1", Device::new_thermometer())
            .is_some());
        assert!(home.add_device("R2", "T1", Device::new_socket()).is_none());
        assert_eq!(
            &Device::new_socket(),
            home.get_device_by_path("R1", "S1").unwrap()
        );
        assert_eq!(
            &Device::new_socket(),
            home.get_device_by_path("R1", "S2").unwrap()
        );
        assert_eq!(
            &Device::new_thermometer(),
            home.get_device_by_path("R1", "T").unwrap()
        );
        assert_eq!(
            &Device::new_socket(),
            home.get_device_by_path("R2", "S1").unwrap()
        );
        assert_eq!(
            &Device::new_thermometer(),
            home.get_device_by_path("R1", "T1").unwrap()
        );
        assert!(home.get_device_by_path("R1", "No device").is_none());
        assert!(home.get_device_by_path("R2", "T").is_none());
    }

    #[test]
    fn test_remove_device() {
        let mut home = Home::new("Home for devices");
        assert!(home.add_room("R1").is_some());
        assert!(home.add_room("R2").is_some());
        assert!(home.add_device("R1", "S1", Device::new_socket()).is_some());
        assert!(home.add_device("R1", "S2", Device::new_socket()).is_some());
        assert!(home
            .add_device("R1", "T", Device::new_thermometer())
            .is_some());
        assert!(home
            .add_device("R1", "S1", Device::new_thermometer())
            .is_none());
        assert_eq!(3, home.device_names_list("R1").len());
        assert!(home.add_device("R2", "S1", Device::new_socket()).is_some());
        assert!(home
            .add_device("R2", "T1", Device::new_thermometer())
            .is_some());
        assert_eq!(2, home.device_names_list("R2").len());
        assert!(home.remove_device("R1", "No device").is_none());
        assert!(home.remove_device("R1", "S1").is_some());
        assert!(home.remove_device("R1", "S2").is_some());
        assert!(home.remove_device("R1", "T1").is_none());
        assert!(home.remove_device("R1", "T").is_some());
        assert!(home.device_names_list("R1").is_empty());
        assert!(home.remove_device("R2", "S1").is_some());
        assert!(home.remove_device("R2", "T1").is_some());
        assert!(home.device_names_list("R2").is_empty());
    }
}
