
use airquality::scd30::SCD30;
use std::{thread, time};


fn main() {
    let mut sensor = SCD30::new().unwrap();
    let speed = sensor.get_bus_speed().unwrap();
    println!("bus Speed: {}", speed);


    sensor.start().unwrap();
    sensor.set_measure_interval(2).unwrap();


    let version = sensor.read_firmware_version().unwrap();
    println!("Current firmware version {}", version);

    sensor.read_measure().unwrap();

    thread::sleep(time::Duration::from_millis(3000));

    let iv = sensor.read_measure_interval().unwrap();
    println!("Current read intervall {}", iv);

    if sensor.data_available().unwrap() == true {
        println!("Data avail");
    } else {
        println!("No avail");
    }
    //sensor.stop().unwrap();

    let speed = sensor.get_bus_speed().unwrap();
    println!("bus Speed: {}", speed);
}
