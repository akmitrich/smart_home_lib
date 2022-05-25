#[allow(dead_code, unused)]
mod smart_home {
    use crate::smart_room::Room;
    use std::collections::{HashMap, hash_map::Values};

    pub struct Home {
        name: String,
        rooms: HashMap<String, Room>,
    }

    impl<'a> Home {
        pub fn new(name: &str) -> Self {
            Home {
                name: String::from(name),
                rooms: HashMap::new(),
            }
        }

        pub fn room_list(&self) -> Values<String, Room> {
            todo!()
        }

        pub fn add_room(&mut self, room: Room) {
            todo!()
        }

        pub fn remove_room(&mut self, unique_name: &str) -> Option<Room>
        {
            todo!()
        }

        pub fn report(&self) -> String {
            todo!()
        }
    }
}

#[allow(dead_code, unused)]
mod smart_room {
    use std::collections::{HashMap, hash_map::Values};
    use crate::smart_device::Device;

    pub struct Room {
        unique_name: String,
        devices: HashMap<String, Device>,
    }

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
}

#[allow(unused, dead_code)]
mod smart_device {
    pub enum Device {
        Unknown,
    }

    impl Device {
        pub fn description(&self) -> String {
            match self {
                Device::Unknown => String::from("Unknown device."),
            }
        }
    }
}

#[cfg(test)]
mod tests {}
