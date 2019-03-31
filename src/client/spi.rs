
use embedded_hal::blocking::spi;

use futures::prelude::*;

use crate::common::*;
use crate::error::Error;
use super::{Mux, Requester};

pub struct Spi {
    device: String,
    mux: Mux,
}

impl Spi {
    pub (crate) fn new(device: String, mux: Mux) -> Self {
        Spi{device, mux}
    }
}

impl spi::Transfer<u8> for Spi {
    type Error = Error;

    fn transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<&'w [u8], Error> {
        debug!("spi transfer request {}", self.device);
        let resp = self.mux.do_request(&self.device, RequestKind::SpiTransfer(Data{data: data.to_vec()})).wait()?;
        debug!("spi transfer response");
        match resp {
            ResponseKind::SpiTransfer(d) => {
                data.clone_from_slice(&d);
                Ok(data)
            },
            _ => Err(Error::InvalidResponse(resp)),
        }
    }
}

impl spi::Write<u8> for Spi {
    type Error = Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let resp = self.mux.do_request(&self.device, RequestKind::SpiWrite(Data{data: data.to_vec()})).wait()?;
        match resp {
            ResponseKind::Ok => Ok(()),
            _ => Err(Error::InvalidResponse(resp)),
        }
    }
}


