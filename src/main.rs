
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

    thread::sleep(time::Duration::from_millis(3000));

    let temperature = sensor.temperature().unwrap();
    let co2 = sensor.co2().unwrap();
    let humidity = sensor.humidity().unwrap();

    println!(
        "co2 = {:.0} ppm, temp = {:.2} °C, humidity = {:.0} %",
       co2, temperature, humidity
    );

}
