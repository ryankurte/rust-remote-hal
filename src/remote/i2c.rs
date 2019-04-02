
use embedded_hal::blocking::i2c;

use futures::prelude::*;

use crate::common::*;
use crate::error::Error;
use super::{Mux, Requester};

pub struct I2c {
    device: String,
    mux: Mux,
}

impl I2c {
    pub (crate) fn new(device: String, mux: Mux) -> Self {
        I2c{device, mux}
    }
}

impl i2c::Read for I2c {
    type Error = Error;

    fn read(&mut self, addr: u8, buff: &mut [u8]) -> Result<(), Error> {
        let resp = self.mux.do_request(&self.device, RequestKind::I2cRead(I2cRead{addr, read_len: buff.len() as u16})).wait()?;
        match resp {
            ResponseKind::I2cRead(d) => {
                buff.clone_from_slice(&d);
                Ok(())
            },
            _ => Err(Error::InvalidResponse(resp)),
        }
    }
}

impl i2c::Write for I2c {
    type Error = Error;

    fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        let resp = self.mux.do_request(&self.device, RequestKind::I2cWrite(I2cWrite{addr, write_data: Data{data: data.to_vec()}})).wait()?;
        match resp {
            ResponseKind::Ok => Ok(()),
            _ => Err(Error::InvalidResponse(resp)),
        }
    }
}


impl i2c::WriteRead for I2c {
    type Error = Error;

    fn write_read(&mut self, addr: u8, data: &[u8], buff: &mut [u8]) -> Result<(), Error> {
        let resp = self.mux.do_request(&self.device, RequestKind::I2cWriteRead(I2cWriteRead{addr, write_data: Data{data: data.to_vec()}, read_len: buff.len() as u16})).wait()?;
        match resp {
            ResponseKind::I2cRead(d) => {
                buff.clone_from_slice(&d);
                Ok(())
            },
            _ => Err(Error::InvalidResponse(resp)),
        }
    }
}