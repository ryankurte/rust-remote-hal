
use linux_embedded_hal::{I2cdev, i2cdev::linux::LinuxI2CError};

use crate::error::Error;

pub struct I2c {
    dev: I2cdev,
}

impl I2c {
    pub fn new(path: &str) -> Result<Self, Error> {
        let dev = I2cdev::new(path)
            .map_err(|e| Error::Remote(format!("{:?}", e)) )?;

        Ok(Self{dev})
    }
}

use embedded_hal::blocking::i2c;

impl i2c::Read for I2c {
    type Error = LinuxI2CError;

    fn read(&mut self, addr: u8, buff: &mut [u8]) -> Result<(), Self::Error> {
        self.dev.read(addr, buff)
    }
}

impl i2c::Write for I2c {
    type Error = LinuxI2CError;

    fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Self::Error> {
        self.dev.write(addr, data)
    }
}


impl i2c::WriteRead for I2c {
    type Error = LinuxI2CError;

    fn write_read(&mut self, addr: u8, data: &[u8], buff: &mut [u8]) -> Result<(), Self::Error> {
        self.dev.write_read(addr, data, buff)
    }
}