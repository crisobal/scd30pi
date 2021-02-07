use rppal::i2c::I2c;

use std::f32::NAN;
use std::result::Result;
use std::time::Instant;
use std::{error, fmt, thread, time};
use log::{trace,debug};

const CMD_START_CONTINUOUS_MEASUREMENT: u16 = 0x0010;
const CMD_STOP_CONTINUOUS_MEASUREMENT: u16 = 0x0104;
const CMD_SET_MEASUREMENT_INTERVAL: u16 = 0x4600;
const CMD_GET_DATA_READY: u16 = 0x0202;
const CMD_GET_MEASUREMENT: u16 = 0x0300;
const CMD_AUTOMATIC_SELF_CALIBRATION: u16 = 0x5306;
const CMD_SET_FORCED_RECALIBRATION_FACTOR: u16 = 0x5204;
const CMD_SET_TEMPERATURE_OFFSET: u16 = 0x5403;
const CMD_SET_ALTITUDE_COMPENSATION: u16 = 0x5102;
const CMD_RESET: u16 = 0xD304;
const CMD_GET_FIRMWARE_VERSION: u16 = 0xD100;

#[derive(Debug)]
pub enum Error {
    I2c(rppal::i2c::Error),
    NoData(String),
    CrcError(String),
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::I2c(ref err) => write!(f, "I2C error: {}", err),
            Error::NotImplemented => write!(f, "Operation not implemented"),
            Error::NoData(ref s) => write!(f, "NoData {}", s),
            Error::CrcError(ref s) => write!(f, "CrcError {}", s),
        }
    }
}

impl error::Error for Error {}

impl From<rppal::i2c::Error> for Error {
    fn from(err: rppal::i2c::Error) -> Error {
        Error::I2c(err)
    }
}

pub struct SCD30 {
    interval_in_s: u16,
    i2c: I2c,
    temperature: f32,
    humidity: f32,
    co2: f32,
    last_read_time: Option<Instant>,
}

impl SCD30 {
    pub fn new() -> Result<SCD30, Error> {
        SCD30::from_slave_address(0x61) // 0x61
    }

    pub fn from_slave_address(slave_address: u16) -> Result<SCD30, Error> {
        let res = I2c::new();
        match res {
            Ok(mut an_i2c) => match an_i2c.set_slave_address(slave_address) {
                Err(e) => Err(Error::from(e)),
                Ok(_) => {
                    let mut sensor = SCD30 {
                        i2c: an_i2c,
                        interval_in_s: 2,
                        temperature: NAN,
                        humidity: NAN,
                        co2: NAN,
                        last_read_time: None,
                    };
                    let _ = sensor.read_measure_interval()?;

                    Ok(sensor)
                }
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn get_bus_speed(&mut self) -> Result<u32, Error> {
        match self.i2c.clock_speed() {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn set_measure_interval(&mut self, interval_seconds: u16) -> Result<(), Error> {
        self.interval_in_s = interval_seconds;
        let _res =
            self.send_cmd_with_args(CMD_SET_MEASUREMENT_INTERVAL, interval_seconds)?;
        Ok(())
    }

    pub fn read_firmware_version(&mut self) -> Result<String, Error> {
        let res = self.read_u16_with_crc(CMD_GET_FIRMWARE_VERSION)?;
        Ok(format!("{}.{}", (res >> 8), (res & 0xff)))
    }

    pub fn read_measure_interval(&mut self) -> Result<u16, Error> {
        let res = self.read_u16_with_crc(CMD_SET_MEASUREMENT_INTERVAL)?;
        self.interval_in_s = res;
        Ok(res)
    }

    pub fn read_measure(&mut self) -> Result<u16, Error> {
        if self.last_read_time == None
            || self.last_read_time.unwrap().elapsed().as_secs() > self.interval_in_s as u64
        {
            if self.data_available()? {
                let mut buf = [0u8; 18];
                let res = self.read_data(CMD_GET_MEASUREMENT, &mut buf)?;
                if res != 18 {
                    return Err(Error::NoData("Expected 18 bytes of data".to_string()));
                }
                trace!("Got {} bytes of measure data: {:x?}", res, buf);

                self.co2 = decode_measure_value_to_u32(&buf[0..6])?;
                self.temperature = decode_measure_value_to_u32(&buf[6..12])?;
                self.humidity = decode_measure_value_to_u32(&buf[12..18])?;

                debug!(
                    "co2 = {:.0} ppm, temp = {:.2} °C, humidity = {:.0} %",
                    self.co2, self.temperature, self.humidity
                );

                self.last_read_time = Some(Instant::now());
                return Ok(res as u16);
            }
        }
        Ok(0)
    }

    pub fn temperature(&mut self) -> Result<f32, Error> {
        self.read_measure()?;
        Ok(self.temperature)
    }

    pub fn humidity(&mut self) -> Result<f32, Error> {
        self.read_measure()?;
        Ok(self.humidity)
    }

    pub fn co2(&mut self) -> Result<f32, Error> {
        self.read_measure()?;
        Ok(self.co2)
    }

    pub fn enable_self_calibration(&mut self) -> Result<(), Error> {
        let _res = self.send_cmd_with_args(CMD_AUTOMATIC_SELF_CALIBRATION, 1)?;
        Ok(())
    }

    pub fn disable_self_calibration(&mut self) -> Result<(), Error> {
        let _res = self.send_cmd_with_args(CMD_AUTOMATIC_SELF_CALIBRATION, 0)?;
        Ok(())
    }

    pub fn set_altitude_compensation(&mut self, altitude_mum: u16) -> Result<(), Error> {
        let _res = self.send_cmd_with_args(CMD_SET_ALTITUDE_COMPENSATION, altitude_mum)?;
        Ok(())
    }

    pub fn set_forced_recalibration(&mut self, real_co2_ppm: u16) -> Result<(), Error> {
        let _res =
            self.send_cmd_with_args(CMD_SET_FORCED_RECALIBRATION_FACTOR, real_co2_ppm)?;
        Ok(())
    }

    pub fn set_temperature_offset(&mut self, temp: f32) -> Result<(), Error> {
        let ticks = (temp * 100f32) as u16;
        let _res = self.send_cmd_with_args(CMD_SET_TEMPERATURE_OFFSET, ticks)?;
        Ok(())
    }

    pub fn start_with_alt_comp(&mut self, pressure_mbar: u16) -> Result<(), Error> {
        let _res =
            self.send_cmd_with_args(CMD_START_CONTINUOUS_MEASUREMENT, pressure_mbar)?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let _res = self.send_cmd_with_args(CMD_START_CONTINUOUS_MEASUREMENT, 0)?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        let _res = self.send_cmd(CMD_STOP_CONTINUOUS_MEASUREMENT)?;
        Ok(())
    }

    pub fn soft_reset(&mut self) -> Result<(), Error> {
        let _res = self.send_cmd(CMD_RESET)?;
        Ok(())
    }

    pub fn data_available(&mut self) -> Result<bool, Error> {
        let res = self.read_u16_with_crc(CMD_GET_DATA_READY)?;
        match res {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    fn send_cmd(&mut self, command: u16) -> Result<(), Error> {
        let buf = prepare_cmd(command);

        match self.i2c.write(&buf) {
            Err(e) => Err(Error::from(e)),
            _ => Ok(()),
        }
    }

    fn send_cmd_with_args(&mut self, command: u16, arguments: u16) -> Result<(), Error> {
        let buf = prepare_cmd_with_args(command, arguments);
        match self.i2c.write(&buf) {
            Err(e) => Err(Error::from(e)),
            _ => Ok(()),
        }
    }

    #[allow(dead_code)]
    fn read_u16(&mut self, command: u16) -> Result<u16, Error> {
        self.send_cmd(command)?;

        thread::sleep(time::Duration::from_millis(5));

        let mut rcv_buf = [0u8; 2];

        let res = self.i2c.read(&mut rcv_buf);
        match res {
            Err(_) => {
                return Err(Error::NoData("No data read".to_string()));
            }
            Ok(s) => {
                if s != 2 {
                    return Err(Error::NoData("Invalid data count read".to_string()));
                }
            }
        }
        let response: u16 = ((rcv_buf[0] as u16) << 8) as u16 + rcv_buf[1] as u16;
        trace!("Read {} raw {:x?}", response, rcv_buf);
        Ok(response)
    }

    fn read_u16_with_crc(&mut self, command: u16) -> Result<u16, Error> {
        self.send_cmd(command)?;

        thread::sleep(time::Duration::from_millis(5));

        let mut rcv_buf = [0u8; 3];

        let res = self.i2c.read(&mut rcv_buf);
        match res {
            Err(_) => {
                return Err(Error::NoData("No data read".to_string()));
            }
            Ok(s) => {
                if s != 3 {
                    return Err(Error::NoData("Invalid data count read".to_string()));
                }
            }
        }
        if calculate_crc8(&rcv_buf) != 0 {
            return Err(Error::CrcError("Invalid in result word".to_string()));
        }
        let response: u16 = ((rcv_buf[0] as u16) << 8) as u16 + rcv_buf[1] as u16;
        trace!("Read {} raw {:#x?}", response, rcv_buf);
        Ok(response)
    }

    fn read_data(&mut self, command: u16, out_buf: &mut [u8]) -> Result<usize, Error> {
        self.send_cmd(command)?;

        thread::sleep(time::Duration::from_millis(5));

        let res = self.i2c.read(out_buf);

        match res {
            Err(_) => Err(Error::NoData("No data read".to_string())),
            Ok(s) => Ok(s),
        }
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

pub fn prepare_cmd(command: u16) -> Vec<u8> {
    let mut res_buf = Vec::<u8>::with_capacity(2);
    res_buf.push((command >> 8) as u8);
    res_buf.push((command & 0xff) as u8);
    res_buf
}

pub fn prepare_cmd_with_args(command: u16, arguments: u16) -> Vec<u8> {
    let arg_buffer = [(arguments >> 8) as u8, (arguments & 0xff) as u8];
    prepare_cmd_with_buf(command, &arg_buffer, true)
}

pub fn prepare_cmd_with_buf(command: u16, buf: &[u8], with_crc: bool) -> Vec<u8> {
    let mut res_buf = Vec::<u8>::with_capacity(buf.len() + 3);
    res_buf.push((command >> 8) as u8);
    res_buf.push((command & 0xff) as u8);
    res_buf.extend_from_slice(buf);

    if with_crc && buf.len() > 0 {
        res_buf.push(calculate_crc8(buf));
    }
    trace!("Buf for cmd 0x{:0x} : {:0x?}", command, res_buf);
    res_buf
}

pub fn decode_measure_value_to_u32(data: &[u8]) -> Result<f32, Error> {
    if calculate_crc8(&data[0..3]) == 0 && calculate_crc8(&data[3..6]) == 0 {
        let mut val: u32 = data[0] as u32;
        val <<= 8;
        val |= data[1] as u32;
        val <<= 8;
        val |= data[3] as u32;
        val <<= 8;
        val |= data[4] as u32;
        Ok(f32::from_bits(val))
    } else {
        Err(Error::CrcError("Invalid CRC".to_string()))
    }
}

pub fn calculate_crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xff;
    for b in data {
        crc ^= b;
        (0..8).for_each(|_| {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        });
    }
    crc
}

#[cfg(test)]
mod tests;
