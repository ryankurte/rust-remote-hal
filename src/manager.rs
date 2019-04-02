
use futures::prelude::*;

use crate::common::*;
use crate::error::Error;

pub trait Manager {
    type Spi;
    type I2c;
    type Pin;

    fn spi(&mut self, path: &str, baud: u32, mode: SpiMode) -> Box<Future<Item=Self::Spi, Error=Error>>;
    fn pin(&mut self, path: &str, mode: PinMode) -> Box<Future<Item=Self::Pin, Error=Error>>;
    fn i2c(&mut self, path: &str) -> Box<Future<Item=Self::I2c, Error=Error>>;
}
