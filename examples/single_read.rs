/*
MIT License

Copyright (c) 2021 Crispin Tschirky <ct@fhr.ch>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */
use scd30pi::i2c::SCD30;
use std::{thread, time};

fn main() {
    let mut sensor = SCD30::new().unwrap();
    let speed = sensor.get_bus_speed().unwrap();
    println!("bus Speed: {}", speed);

    sensor.start().unwrap();
    sensor.set_measure_interval(2).unwrap();

    let version = sensor.read_firmware_version().unwrap();
    println!("Current firmware version {}", version);

    while !sensor.data_available().unwrap() {
        thread::sleep(time::Duration::from_millis(200));
    }

    let temperature = sensor.temperature().unwrap();
    let co2 = sensor.co2().unwrap();
    let humidity = sensor.humidity().unwrap();

    println!(
        "co2 = {:.0} ppm, temp = {:.2} °C, humidity = {:.0} %",
        co2, temperature, humidity
    );
}
