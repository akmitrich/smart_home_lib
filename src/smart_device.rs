
#![allow(unused, dead_code)]
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
