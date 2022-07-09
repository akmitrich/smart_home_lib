use smart_home::home;
use smart_home::smart_device;
fn main() {
    average_temperature_in_room();
    report_example()
}

fn average_temperature_in_room() {
    let mut home = home::Home::new("Home");
    home.add_room("Main");
    home.add_device(
        "Main",
        "Thermo1",
        smart_device::Thermometer::new(24_f64).into(),
    );
    home.add_device(
        "Main",
        "Thermo2",
        smart_device::Thermometer::new(26_f64).into(),
    );
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
        println!(
            "Average temperature in Main room is {}",
            average_temperature
        );
    }
}

fn report_example() {
    let mut home = home::Home::new("Home");
    home.add_room("Main");
    home.add_device(
        "Main",
        "Thermo1",
        smart_device::Thermometer::new(24_f64).into(),
    );
    home.add_device(
        "Main",
        "Thermo2",
        smart_device::Thermometer::new(26_f64).into(),
    );
    home.add_device(
        "Main",
        "S1",
        smart_device::Socket::new(220., 5., true).into(),
    );
    home.add_device(
        "Main",
        "S2",
        smart_device::Socket::new(220., 0., false).into(),
    );

    home.add_room("Kitchen");
    home.add_device("Kitchen", "Sock", smart_device::Device::new_socket());
    home.add_device("Kitchen", "Thermo", smart_device::Device::new_thermometer());
    println!("Generate report ... \n\n{}", home.report());
}
