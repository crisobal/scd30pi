use rppal::i2c::I2c;

struct SCD30 {
    i2c : I2c
}

impl SCD30 {
    pub fn from_slave_address( slave_address : u16) -> SCD30 {
        let mut an_i2c = I2c::new()?;
        an_i2c.set_slave_address(slave_address);
        SCD30 {
            i2c : an_i2c
        }
    }
    pub fn readMeasurement(&self) -> bool {
        false
    }
}

/*
          Name:  CRC-8
Protected Data: read data
         Width: 8 bits
    Polynomial: 0x31 (x⁸ + x⁵ + x⁴ + x⁰)
Initialization: 0xFF
 Reflect Input: false
Reflect Output: false
     Final XOR: 0x00
       Example: CRC(0xBEEF) = 0x92
          From: http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html
   Tested with: http://www.sunshine2k.de/coding/javascript/crc/crc_js.html

*/
pub fn calculate_crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xff;
    for b in data {
        crc ^= b;
        (0..8).for_each(
            |f | {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ 0x31;
                } else {
                    crc <<= 1;
                }
            },
        );
    }
    crc
}

#[cfg(test)]
mod tests;
