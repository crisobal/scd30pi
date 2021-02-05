
use airquality::scd30::SCD30;


fn main() {
    let mut sensor = SCD30::new().unwrap();
    //sensor.stop();
    sensor.set_measure_interval(10000);
    sensor.start();
    let iv = sensor.read_measure_interval().unwrap();
    println!("Hello, world! {}", iv);

    if(sensor.data_available().unwrap()){
        println!("Data avail");
    } else {
        println!("No avail");
    }
    //sensor.stop().unwrap()
    let speed = sensor.get_bus_speed().unwrap();
    println!("bus Speed: {}", speed);
}
