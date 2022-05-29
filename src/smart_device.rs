
#![allow(unused, dead_code)]
pub enum Device {
    Unknown,
}

impl Device {
    pub fn report_state(&self) -> String {
        match self {
            Device::Unknown => String::from("Unknown device."),
        }
    }
}
