CO2 Sensor SCD30 for Rust
=========================

This is yet another implementation of a Sensirion SCD30 i2c sensor driver in 
pure rust. The sensor measures CO2, temperature and humidity.

The sensor documentation can be obtained directly from Sensirion: 
https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.5_CO2/Sensirion_CO2_Sensors_SCD30_Interface_Description.pdf

Please take in account that this is my first project in Rust since reading the book so it might not be as rusty
as it should be as my Java and C++ background influenced my code non negligible. 

The target platform is a Raspberry pi so either you compile it on an rpi or use crosscompiling.
It is based on Raspberry Pi Peripheral Access Library (https://crates.io/crates/rppal). The 
physical SCD30 connection to i2c can be done by wiring the device to the first i2c ports of the GPIO
connector.

Connect to RaspberryPi GPIO:
- pin 1 (3.3V/VCC)
- pin 3 (SDA)
- pin 5 (SCL)
- pin 6 (GND).

Example
-------

```rust
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
```


Cross Compile
-------------

- **Install the arm cross compiler.** You can find the crosscompiler on
  https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads
  you can unpack the content to the directory of your choice. Assume it is `TARGET_DIR` then export
  following variables prior to use cargo to cross compile:

  `export CC_armv7_unknown_linux_gnueabihf=arm-none-linux-gnueabihf-gcc`
  `export PATH=${TARGET_DIR}/gcc-arm-10.2-2020.11-x86_64-arm-none-linux-gnueabihf/bin:$PATH`

- Install rust cross compile target.

  `rustup target add armv7-unknown-linux-gnueabihf`

- Configure cargo for the cross compiler target. Add following to `~/.cargo/config`

  `[target.armv7-unknown-linux-gnueabihf]`

  `linker = "arm-linux-gnueabihf-gcc"`

Copile using:

`cargo build --target=armv7-unknown-linux-gnueabihf --release`

Additional resources about cross compiling can be found at https://github.com/japaric/rust-cross

