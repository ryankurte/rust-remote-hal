
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

impl std::ops::Deref for Spi {
    type Target = Spidev;

    fn deref(&self) -> &Self::Target {
        &self.dev
    }
}

impl std::ops::DerefMut for Spi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dev
    }
}