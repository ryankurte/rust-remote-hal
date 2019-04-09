
use std::net::{SocketAddr};
use std::time::Duration;

use futures::prelude::*;
use tokio::prelude::*;

use daemon_engine::{TcpConnection};
use daemon_engine::codecs::json::{JsonCodec};
use rr_mux::{Mux as BaseMux, Connector};

use crate::common::*;
use crate::manager::Manager;
use crate::error::Error;

pub mod spi;
use spi::Spi;
pub mod i2c;
use i2c::I2c;
pub mod pin;
use pin::Pin;

type Mux = BaseMux<u64, (), Request, Response, Error, ()>;


pub const TIMEOUT: Duration = Duration::from_secs(3);

/// Remote client for connecting to remote-hal server peripherals
/// 
/// THIS MUST BE RUN IN A MULTI-THREADED TOKIO CONTEXT
pub struct Client {
    connection: TcpConnection<JsonCodec<Request, Response, Error>>,
    mux: Mux,
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

pub trait Requester {
    fn do_request(&mut self, path: &str, req: RequestKind) -> Box<Future<Item=ResponseKind, Error=Error> + Send + 'static>;
}

impl Requester for Mux {
    fn do_request(&mut self, path: &str, req: RequestKind) -> Box<Future<Item=ResponseKind, Error=Error> + Send + 'static> {
        let req = Request::new(path.to_owned(), req);
        info!("sending request {:?}", req);
        Box::new(self.request((), req.id, (), req)
        .timeout(TIMEOUT)
        .map_err(|e| e.into() )
        .then(|r| {
            let resp = match r {
                Err(e) => return Err(e),
                Ok(v) => v,
            };

            info!("received response {:?}", resp);

            match resp.0.kind {
                ResponseKind::Error(e) => Err(Error::Remote(e)),
                _ => Ok(resp.0.kind),
            }
        }))
    }
}

impl Client {
    /// Create a new remote-hal instance
    pub fn new(addr: SocketAddr) -> impl Future<Item=Self, Error=Error> {
        info!("client connecting to: {}", addr);

        TcpConnection::<JsonCodec<Request, Response, Error>>::new(&addr, JsonCodec::new()).map_err(|e| e.into() ).timeout(TIMEOUT).map_err(|e| e.into() ).map(|connection| {

            let (tx, rx) = connection.clone().split();

            info!("client connected");

            let mux = Mux::new();

            // Map mux output to tx
            let m = mux.clone();
            let tx_handle = tx.send_all(m.map(|(_req_id, _target, msg, _ctx)| msg.req().unwrap() ).map_err(|e| panic!(e) ));
            tokio::spawn(tx_handle.map(|_v| () ).map_err(|e| panic!(e) ));

            // Map rx to mux input
            let mut m = mux.clone();
            let rx_handle = rx.for_each(move |resp| m.handle_resp(resp.id, (), resp, ()) );
            tokio::spawn(rx_handle.map(|_v| () ).map_err(|e| panic!(e) ));
            
            Self{connection, mux}

        })
    }

    // Close a remote-hal instance
    pub fn close(self) {
        self.connection.close();
    }

    /// Pass-through for raw requests
    pub fn request(&mut self, device: &str, request: RequestKind) -> impl Future<Item=ResponseKind, Error=Error> {
        self.mux.do_request(device, request)
    }
}

pub enum InitRequest{
    Spi{path: String, baud: u32, mode: SpiMode},
    Pin{path: String, mode: PinMode},
    I2c{path: String},
}

pub enum InitResponse {
    Spi(Spi),
    Pin(Pin),
    I2c(I2c),
}

impl Client {
    fn init(&mut self, requests: &[InitRequest]) -> impl Future<Item=Vec<InitResponse>, Error=Error> {
        let f: Vec<_> = requests.iter().map(|r| {
            match r {
                InitRequest::Spi{path, baud, mode} => {
                    //self.spi(&path, *baud, *mode).map(|v| InitResponse::Spi(v) )
                },
                InitRequest::Pin{path, mode} => {
                    //self.pin(&path, *mode).map(|v| InitResponse::Pin(v) )
                },
                InitRequest::I2c{path} => {
                    //self.i2c(&path).map(|v| InitResponse::I2c(v) )
                },
            }
        }).collect();

        future::join_all(f)
    }
}

impl Manager for Client {
    type Spi = Spi;
    type Pin = Pin;
    type I2c = I2c;

    /// Connect to a new Spi instance
    fn spi(&mut self, path: &str, baud: u32, mode: SpiMode) -> Box<Future<Item=Spi, Error=Error> + Send> {
        debug!("attempting connection to SPI device: {}", path);
        let device = path.to_owned();
        let mux = self.mux.clone();
        Box::new(self.mux.do_request(path, RequestKind::SpiConnect(SpiConnect{baud, mode}))
        .then(|res| {
            let resp = match res {
                Err(e) => return Err(e),
                Ok(r) => r,
            };
            match resp {
                ResponseKind::Ok => Ok(Spi::new(device, mux)),
                _ => Err(Error::InvalidResponse(resp)),
            }
        }))
    }

    /// Connect to a new Pin instance
    fn pin(&mut self, path: &str, mode: PinMode) -> Box<Future<Item=Pin, Error=Error> + Send> {
        debug!("attempting connection to Pin: {}", path);
        let device = path.to_owned();
        let mux = self.mux.clone();
        Box::new(self.mux.do_request(path, RequestKind::PinConnect(mode))
        .then(|res| {
            let resp = match res {
                Err(e) => return Err(e),
                Ok(r) => r,
            };
            match resp {
                ResponseKind::Ok => Ok(Pin::new(device, mux)),
                _ => Err(Error::InvalidResponse(resp)),
            }
        }))
    }

    /// Connect to a new I2c instance
    fn i2c(&mut self, path: &str) -> Box<Future<Item=I2c, Error=Error> + Send> {
        debug!("attempting connection to I2c: {}", path);
        let device = path.to_owned();
        let mux = self.mux.clone();
        Box::new(self.mux.do_request(path, RequestKind::I2cConnect)
        .then(|res| {
            let resp = match res {
                Err(e) => return Err(e),
                Ok(r) => r,
            };
            match resp {
                ResponseKind::Ok => Ok(I2c::new(device, mux)),
                _ => Err(Error::InvalidResponse(resp)),
            }
        }))
    }
}
