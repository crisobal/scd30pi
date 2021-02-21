CO2 Sensor SCD30 for Rust
=========================

This yet another implementation of a client for the Sensirion i2c sensor in 
pure rust.

Please take in account that this is my first project in Rust since reading the book so it might be not as rusty
as it should be as my Java and C++ background breaks influenced my code non negligible. 

The target platform is a Raspberry pi so either you compile it on an rpi or use crosscompiling.
The SCD30 connection to i2c can be done by adding the device to the first i2c ports of the GPIO
connector.

Connect to RaspberryPi GPIO:
- pin 1 (3.3V/VCC)
- pin 3 (SDA)
- pin 5 (SCL)
- pin 6 (GND).


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

  [target.armv7-unknown-linux-gnueabihf]

  linker = "arm-linux-gnueabihf-gcc"`

Copile using:

`cargo build --target=armv7-unknown-linux-gnueabihf --release`

Additional resources about cross compiling can be found at https://github.com/japaric/rust-cross

