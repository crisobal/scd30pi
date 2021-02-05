use rppal::i2c::I2c;


use std::result::Result;
use std::{error, fmt};


const COMMAND_START_CONTINUOUS_MEASUREMENT: u16 = 0x0010;
const COMMAND_STOP_CONTINUOUS_MEASUREMENT: u16 = 0x0104;
const COMMAND_SET_MEASUREMENT_INTERVAL         : u16 = 0x4600;
const COMMAND_GET_DATA_READY                   : u16 = 0x0202;
const COMMAND_READ_MEASUREMENT : u16 = 0x0300;
const COMMAND_AUTOMATIC_SELF_CALIBRATION : u16 = 0x5306;
const COMMAND_SET_FORCED_RECALIBRATION_FACTOR : u16 = 0x5204;
const COMMAND_SET_TEMPERATURE_OFFSET : u16 = 0x5403;
const COMMAND_SET_ALTITUDE_COMPENSATION : u16 = 0x5102;
const COMMAND_RESET : u16 = 0xD304;


#[derive(Debug)]
pub enum Error {
    I2c(rppal::i2c::Error),
    NoData(String),
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::I2c(ref err) => write!(f, "I2C error: {}", err),
            Error::NotImplemented => write!(f, "Operation not implemented"),
            Error::NoData(ref s) => write!(f, "NoData {}", s),
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
    i2c: I2c,
}

impl SCD30 {
    pub fn new() -> Result<SCD30, Error>{
        SCD30::from_slave_address(0x61) // 0x61
    }

    pub fn from_slave_address(slave_address: u16) -> Result<SCD30, Error> {
        let res = I2c::new();
        match res {
            Ok(mut an_i2c) => {
                match an_i2c.set_slave_address(slave_address) {
                    Err(e) => Err(Error::from(e)),
                    Ok(_) => Ok(SCD30 { i2c: an_i2c }),
                }
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn get_bus_speed(&mut self) -> Result<u32, Error>{
        match  self.i2c.clock_speed(){
            Ok(s) => {
                Ok(s)
            }
            Err(e) => {
                Err( Error::from(e))
            }
        }
    }

    pub fn set_measure_interval(&mut self, interval_seconds : u16)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_SET_MEASUREMENT_INTERVAL, interval_seconds)?;
        Ok(())
    }

    pub fn read_measure_interval(&mut self)-> Result<u16,Error> {
        let res = self.read_u16(COMMAND_SET_MEASUREMENT_INTERVAL)?;
        Ok(res)
    }

    pub fn enable_self_calibration(&mut self)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_AUTOMATIC_SELF_CALIBRATION, 1)?;
        Ok(())
    }

    pub fn disable_self_calibration(&mut self)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_AUTOMATIC_SELF_CALIBRATION, 0)?;
        Ok(())
    }

    pub fn set_altitude_compensation(&mut self, altitude_mum : u16)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_SET_ALTITUDE_COMPENSATION, altitude_mum)?;
        Ok(())
    }

    pub fn set_forced_recalibration(&mut self, real_co2_ppm : u16)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_SET_FORCED_RECALIBRATION_FACTOR, real_co2_ppm)?;
        Ok(())
    }

    pub fn set_temperature_offset(&mut self, temp : f32)-> Result<(),Error> {
        let ticks = (temp*100f32) as u16;
        let _res = self.send_command_with_args(COMMAND_SET_TEMPERATURE_OFFSET, ticks)?;
        Ok(())
    }

    pub fn start_with_alt_comp(&mut self, pressure_mbar : u16)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_START_CONTINUOUS_MEASUREMENT, pressure_mbar)?;
        Ok(())
    }
    pub fn start(&mut self)-> Result<(),Error> {
        let _res = self.send_command_with_args(COMMAND_START_CONTINUOUS_MEASUREMENT, 0)?;
        Ok(())
    }

    pub fn stop(&mut self)-> Result<(),Error> {
        let _res = self.send_command(COMMAND_STOP_CONTINUOUS_MEASUREMENT)?;
        Ok(())
    }

    pub fn soft_reset(&mut self)-> Result<(),Error> {
        let _res = self.send_command(COMMAND_RESET)?;
        Ok(())
    }

    pub fn data_available(&mut self) -> Result<bool,Error> {
        let res = self.read_u16(COMMAND_GET_DATA_READY)?;
        match res {
            1 => {
                Ok(true)
            }
            _ => Ok(false)
        }
    }

    fn send_command(&mut self, command: u16) -> Result<(),Error> {
        let buf = prepare_cmd(command);

        match self.i2c.write(&buf) {
            Err(e) => {
                Err(Error::from(e))
            },
            _ => Ok(())
        }
    }

    fn send_command_with_args(&mut self, command: u16, arguments: u16) -> Result<(),Error> {
        let buf = prepare_cmd_with_args(command, arguments);
        match self.i2c.write(&buf) {
            Err(e) => {
                Err(Error::from(e))
            },
            _ => Ok(())
        }
    }


    fn read_u16(&mut self, command: u16) -> Result<u16,Error>{
        self.send_command(command)?;

        let mut rcv_buf = [0u8;2];

        let res = self.i2c.read(&mut rcv_buf);
        match res {
            Err(e) => {
                return Err( Error::NoData("No data read".to_string()));
            },
            Ok(s) => {
                if s != 2 {
                    return Err( Error::NoData("Invalid data count read".to_string()));
                }
            },
        }
        let response : u16 = (rcv_buf[0] as u16) << 8 + rcv_buf[1] as u16;
        Ok(response)
    }

    fn read_data(&mut self, command : u16, out_buf : &mut [u8]) -> Result<usize,Error>{
        let snd_buf = prepare_cmd(command);
        match self.i2c.write_read(&snd_buf,out_buf) {
            Err(e) => {
                return Err( Error::NoData("No data read".to_string()) );
            }
            _ => {}
        }

        Ok(0)
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
    let mut res_buf = Vec::<u8>::with_capacity(buf.len()+3);
    res_buf.push((command >> 8) as u8);
    res_buf.push((command & 0xff) as u8);
    res_buf.extend_from_slice( buf);

    if with_crc && buf.len() > 0 {
        res_buf.push(calculate_crc8(buf));
    }
    res_buf
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

