use smart_home_lib::smart_home;
use smart_home_lib::smart_device;
fn main() {
    let mut home = smart_home::Home::new("Home");
    home.add_room("Main");
    home.add_device("Main", "Thermo1",
        smart_device::Thermometer::new(24_f64).into());
    home.add_device("Main", "Thermo2", 
        smart_device::Thermometer::new(26_f64).into());
    if let Some(room) = home.get_room_by_name("Main") {
        let mut temperature = 0_f64;
        let mut count = 0_usize;
        for device in room.device_list() {
            if let smart_device::Device::Thermometer(thermometer) = device {
                count += 1;
                temperature += thermometer.get_temperature();
            }
        }
        let average_temperature = temperature / count as f64;
        println!("Average temperature in Main room is {}", average_temperature);
    }
}