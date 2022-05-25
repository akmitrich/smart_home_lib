use crate::smart_room::Room;
use std::collections::{HashMap, hash_map::Values};

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
