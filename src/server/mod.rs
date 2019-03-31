
use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, hash_map::Entry};
use std::ops::DerefMut;

use daemon_engine::{TcpServer, JsonCodec};
use tokio::prelude::*;

use embedded_hal::blocking::spi::{Transfer as SpiTransfer, Write as SpiWrite};
use embedded_hal::blocking::i2c::{Read as I2cRead, Write as I2cWrite, WriteRead as I2cWriteRead};
use embedded_hal::digital::{InputPin, OutputPin};


use crate::common::*;
use crate::error::Error;

pub mod spi;
use spi::Spi;
pub mod i2c;
use i2c::I2c;
pub mod pin;
use pin::Pin;

#[derive(Clone)]
pub struct Server {
    _server: TcpServer<JsonCodec<Response, Request>>,

    spi: Arc<Mutex<HashMap<String, Spi>>>,
    i2c: Arc<Mutex<HashMap<String, I2c>>>,
    pin: Arc<Mutex<HashMap<String, Pin>>>,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Result<Self, Error> {
        debug!("server binding to: {}", addr);

        let server = TcpServer::<JsonCodec<Response, Request>>::new(&addr, JsonCodec::new()).unwrap();

        let s = Self {
            _server: server.clone(),
            spi: Arc::new(Mutex::new(HashMap::new())),
            i2c: Arc::new(Mutex::new(HashMap::new())),
            pin: Arc::new(Mutex::new(HashMap::new())),    
        };

        let mut s1 = s.clone();

        let server_handle = server.clone()
            .incoming()
            .unwrap()
            .for_each(move |r| {
                let req = r.data();
                info!("Received: {:?} info: {:?}", req, r.info());

                let resp = match s1.handle(&req.device, req.kind) {
                    Ok(resp) => resp,
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                };

                info!("Response: {:?}", resp);

                r.send(Response{id: req.id, kind: resp}).map(|_v| trace!("server send complete") ).map_err(|e| error!("server error: {:?}", e))
            }).map(|v| () ).map_err(|_e| ());

        tokio::spawn(server_handle);

        Ok(s)
    }

    pub fn handle(&mut self, device: &str, req: RequestKind) -> Result<ResponseKind, Error> {
        let resp = match req {
            RequestKind::Ping => ResponseKind::Ok,
            
            RequestKind::SpiConnect(c) => {
                info!("received SpiConnect (device: {}, baud: {}, mode: {})", device, c.baud, c.mode);
                let mut spi_map = self.spi.lock().unwrap();

                match spi_map.entry(device.to_owned()) {
                    Entry::Occupied(_e) => ResponseKind::DeviceAlreadyBound,
                    Entry::Vacant(v) => {
                        v.insert(Spi::new(device, c.baud, c.mode)?);
                        ResponseKind::Ok
                    },
                }
            },

            RequestKind::SpiTransfer{write_data} => {
                info!("received SpiTransfer");
                let mut spi_map = self.spi.lock().unwrap();
                let spi = match spi_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let mut d = write_data.data.clone();

                match SpiTransfer::transfer(spi.deref_mut(), &mut d) {
                    Ok(d) => ResponseKind::SpiTransfer(d.to_vec()),
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::SpiWrite{write_data} => {
                info!("received SpiWrite");
                let mut spi_map = self.spi.lock().unwrap();
                let spi = match spi_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let mut d = write_data.data.clone();

                match SpiWrite::write(spi.deref_mut(), &mut d) {
                    Ok(_) => ResponseKind::Ok,
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::I2cConnect => {
                info!("received I2cConnect (device: {})", device);
                let mut i2c = self.i2c.lock().unwrap();

                match i2c.entry(device.to_owned()) {
                    Entry::Occupied(_e) => ResponseKind::DeviceAlreadyBound,
                    Entry::Vacant(v) => {
                        v.insert(I2c::new(device)?);
                        ResponseKind::Ok
                    },
                }
            },

            RequestKind::I2cWrite(c) => {
                info!("received I2cWrite (address: {}, data: {:?})", c.addr, c.write_data);
                let mut i2c_map = self.i2c.lock().unwrap();
                let i2c = match i2c_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                match I2cWrite::write(i2c.deref_mut(), c.addr, &c.write_data.data) {
                    Ok(_) => ResponseKind::Ok,
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::I2cRead(c) => {
                info!("received I2cRead (address: {}, len: {})", c.addr, c.read_len);
                let mut i2c_map = self.i2c.lock().unwrap();
                let i2c = match i2c_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let mut buff = vec![0; c.read_len as usize];

                match I2cRead::read(i2c.deref_mut(), c.addr, &mut buff) {
                    Ok(_) => ResponseKind::I2cRead(buff),
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::I2cWriteRead(c) => {
                info!("received I2cWriteRead (address: {}, write_data: {:?}, read_len: {}", c.addr, c.write_data, c.read_len);
                let mut i2c_map = self.i2c.lock().unwrap();
                let i2c = match i2c_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let mut buff = vec![0; c.read_len as usize];

                match I2cWriteRead::write_read(i2c.deref_mut(), c.addr, &c.write_data.data, &mut buff) {
                    Ok(_) => ResponseKind::I2cRead(buff),
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::PinConnect => {
                info!("received PinConnect (device: {})", device);
                let mut pin = self.pin.lock().unwrap();

                match pin.entry(device.to_owned()) {
                    Entry::Occupied(_e) => ResponseKind::DeviceAlreadyBound,
                    Entry::Vacant(v) => {
                        v.insert(Pin::new(device)?);
                        ResponseKind::Ok
                    },
                }
            },

            RequestKind::PinSet(c) => {
                info!("received PinSet");
                let mut pin_map = self.pin.lock().unwrap();
                let pin = match pin_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let res: Result<_, ()> = Ok(match c.value {
                    true => OutputPin::set_high(pin.deref_mut()),
                    false => OutputPin::set_low(pin.deref_mut()),
                });

                match res {
                    Ok(_) => ResponseKind::Ok,
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            RequestKind::PinGet => {
                info!("received PinGet");
                let mut pin_map = self.pin.lock().unwrap();
                let pin = match pin_map.get_mut(device) {
                    Some(s) => s,
                    None => return Ok(ResponseKind::DeviceNotBound),
                };

                let v: Result<_, ()> = Ok(InputPin::is_high(pin.deref_mut()));

                match v {
                    Ok(v) => ResponseKind::PinGet(v),
                    Err(e) => ResponseKind::Error(format!("{:?}", e)),
                }
            },

            _ => ResponseKind::Unhandled
        };

        Ok(resp)
    }
}
