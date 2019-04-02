use std::io;

use embedded_hal::blocking::spi;
use linux_embedded_hal::{spidev, Spidev};

use crate::common::*;
use crate::error::Error;

pub struct Spi {
    dev: Spidev,
}

impl Spi {
    pub fn new(path: &str, baud: u32, mode: SpiMode) -> Result<Self, Error> {
        let mut dev = Spidev::open(path)?;

        let mode = match mode {
            SpiMode::Mode0 => spidev::SPI_MODE_0,
            SpiMode::Mode1 => spidev::SPI_MODE_1,
            SpiMode::Mode2 => spidev::SPI_MODE_2,
            SpiMode::Mode3 => spidev::SPI_MODE_3,
        };

        let mut config = spidev::SpidevOptions::new();
        config.max_speed_hz(baud);
        config.mode(mode);

        dev.configure(&config)?;

        Ok(Self{dev})
    }
}

impl spi::Transfer<u8> for Spi {
    type Error = io::Error;

    fn transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.dev.transfer(data)
    }
}

impl spi::Write<u8> for Spi {
    type Error = io::Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.dev.write(data)
    }
}

