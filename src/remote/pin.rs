
use embedded_hal::digital;

use futures::prelude::*;

use crate::common::*;
use crate::error::Error;
use super::{Mux, Requester};

pub struct Pin {
    device: String,
    mux: Mux,
}

impl Pin {
    pub (crate) fn new(device: String, mux: Mux) -> Self {
        Pin{device, mux}
    }

    fn set(&mut self, value: bool) -> Result<(), Error> {
        let resp = self.mux.do_request(&self.device, RequestKind::PinSet(Value{value})).wait()?;
        match resp {
            ResponseKind::Ok => Ok(()),
             _ => Err(Error::InvalidResponse(resp)),
        }
    }

    fn get(&self) -> Result<bool, Error> {
        let mut mux = self.mux.clone();
        let resp = mux.do_request(&self.device, RequestKind::PinGet).wait()?;
        match resp {
            ResponseKind::PinGet(v) => Ok(v),
             _ => Err(Error::InvalidResponse(resp)),
        }
    }
}

impl digital::InputPin for Pin {
    //type Error = Error;

    fn is_high(&self) -> bool {
        self.get().unwrap() == true
    }

    fn is_low(&self) -> bool {
        self.get().unwrap() == false
    }
}

impl digital::OutputPin for Pin {
    //type Error = Error;

    fn set_high(&mut self) {
        self.set(true).unwrap();
    }

    fn set_low(&mut self) {
        self.set(false).unwrap();
    }
}


