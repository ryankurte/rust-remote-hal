
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

impl std::ops::Deref for Pin {
    type Target = PinDev;

    fn deref(&self) -> &Self::Target {
        &self.dev
    }
}

impl std::ops::DerefMut for Pin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dev
    }
}