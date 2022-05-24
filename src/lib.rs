#[allow(dead_code, unused)]
mod smart_home {
    use crate::smart_room::Room;
    use std::collections::HashMap;

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

        pub fn room_list(&self) -> Vec<&'a Room> {
            todo!()
        }

        pub fn add_room(&mut self, room: Room) {
            todo!()
        }

        pub fn remove_room(&mut self, unique_name: &str) /*-> Room ?*/
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
    use std::{collections::HashMap, rc::Rc};

    pub struct Room {
        unique_name: String,
        devices: HashMap<String, Rc<dyn Device>>,
    }

    impl Room {
        pub fn new(unique_name: &str) -> Self {
            Room {
                unique_name: String::from(unique_name),
                devices: HashMap::new(),
            }
        }

        pub fn device_list(&self) -> Vec<Rc<dyn Device>> {
            todo!()
        }

        pub fn add_device(&mut self, device: &dyn Device, unique_name: &str) {
            todo!()
        }

        pub fn remove_device(&mut self, unique_name: &str) /* -> Rc<dyn Device> ?*/
        {
            todo!()
        }
    }

    pub trait Device {
        fn description(&self) -> &str;
    }
}

#[cfg(test)]
mod tests {}
