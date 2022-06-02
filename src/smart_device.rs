#![allow(unused, dead_code)]

use std::fmt::format;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
}

#[derive(Debug, PartialEq)]
pub struct Socket {
    voltage: f64,
    current: f64,
    on: bool,
}

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    temperature: f64,
}

pub trait ReportState {
    fn report_state(&self) -> String;
}

impl Device {
    pub fn new_socket() -> Self {
        Device::Socket(Socket::new(220_f64, 0_f64))
    }

    pub fn new_thermometer() -> Self {
        Device::Thermometer(Thermometer::new(20_f64))
    }
}

impl ReportState for Device {
    fn report_state(&self) -> String {
        match self {
            Device::Socket(s) => format!("Socket {}", s.report_state()),
            Device::Thermometer(t) => format!("Thermometer, {}", t.report_state()),
            _ => String::from("Unknown device."),
        }
    }
}

impl Socket {
    pub fn new(voltage: f64, current: f64) -> Self {
        Self {
            voltage,
            current,
            on: false,
        }
    }

    pub fn voltage(mut self, voltage: f64) -> Self {
        self.voltage = voltage;
        self
    }

    pub fn current(mut self, current: f64) -> Self {
        self.current = current;
        self
    }

    pub fn get_current_power(&self) -> f64 {
        self.current * self.voltage
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn switch(mut self, on: bool) -> Self {
        self.on = on;
        self
    }
}

impl ReportState for Socket {
    fn report_state(&self) -> String {
        format!(
            " is {}; current power is {}",
            if self.is_on() { "on" } else { "off" },
            self.get_current_power()
        )
    }
}

impl Thermometer {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

impl ReportState for Thermometer {
    fn report_state(&self) -> String {
        format!(" temperature is {}", self.get_temperature())
    }
}
