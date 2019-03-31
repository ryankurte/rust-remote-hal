
use linux_embedded_hal::{Spidev};
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};

use crate::error::Error;

pub struct Spi {
    dev: Spidev,
}

impl Spi {
    pub fn new(path: &str, baud: u32, mode: u32) -> Result<Self, Error> {
        let mut dev = Spidev::open(path)?;

        let mode = SpiModeFlags::from_bits_truncate(mode);

        let mut config = SpidevOptions::new();
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