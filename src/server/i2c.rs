
use linux_embedded_hal::{I2cdev};

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

impl std::ops::Deref for I2c {
    type Target = I2cdev;

    fn deref(&self) -> &Self::Target {
        &self.dev
    }
}

impl std::ops::DerefMut for I2c {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dev
    }
}