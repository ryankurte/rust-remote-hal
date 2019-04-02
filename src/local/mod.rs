
use futures::prelude::*;
use futures::future::{ok, err};

use crate::common::*;
use crate::manager::Manager;
use crate::error::Error;

pub mod i2c;
pub use i2c::I2c;
pub mod spi;
pub use spi::Spi;
pub mod pin;
pub use pin::Pin;


/// Fake client impl for connecting to local devices
pub struct Client {
    _reserved: (),
}

impl Client {
    /// Create a new local client instance
    pub fn new() -> impl Future<Item=Client, Error=Error> {
        ok(Client{_reserved: ()})
    }
}

impl Manager for Client {
    type Spi = Spi;
    type Pin = Pin;
    type I2c = I2c;

    /// Connect to a new Spi instance
    fn spi(&mut self, path: &str, baud: u32, mode: SpiMode) -> Box<Future<Item=Spi, Error=Error>+ Send> {
        debug!("attempting connection to SPI device: {}", path);
        let d = match Spi::new(path, baud, mode) {
            Ok(d) => ok(d),
            Err(e) => err(e),
        };
        Box::new(d)
    }

    /// Connect to a new Pin instance
    fn pin(&mut self, path: &str, mode: PinMode) -> Box<Future<Item=Pin, Error=Error> + Send> {
        debug!("attempting connection to Pin: {}", path);
        let d = match Pin::new(path, mode) {
            Ok(d) => ok(d),
            Err(e) => err(e),
        };
        Box::new(d)
    }

    /// Connect to a new I2c instance
    fn i2c(&mut self, path: &str) -> Box<Future<Item=I2c, Error=Error> + Send> {
        debug!("attempting connection to I2c: {}", path);
        let d = match I2c::new(path) {
            Ok(d) => ok(d),
            Err(e) => err(e),
        };
        Box::new(d)
    }
}
