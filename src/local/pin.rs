use embedded_hal::digital;

use linux_embedded_hal::{Pin as PinDev};
use linux_embedded_hal::sysfs_gpio::Direction;

use crate::common::PinMode;
use crate::error::Error;

pub struct Pin {
    dev: PinDev,
}

impl Pin {
    pub fn new(path: &str, mode: PinMode) -> Result<Self, Error> {
        let dev = PinDev::from_path(path)
            .map_err(|e| Error::Remote(format!("{:?}", e)) )?;

        dev.export()?;
        match mode {
            PinMode::Input => dev.set_direction(Direction::In)?,
            PinMode::Output => dev.set_direction(Direction::Out)?,
        }

        Ok(Self{dev})
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        self.dev.unexport().unwrap();
    }
}

impl digital::InputPin for Pin {
    //type Error = Error;

    fn is_high(&self) -> bool {
        self.dev.is_high()
    }

    fn is_low(&self) -> bool {
        self.dev.is_low()
    }
}

impl digital::OutputPin for Pin {
    //type Error = Error;

    fn set_high(&mut self) {
        self.dev.set_high();
    }

    fn set_low(&mut self) {
        self.dev.set_low();
    }
}


