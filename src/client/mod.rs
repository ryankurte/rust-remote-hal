
use std::net::{SocketAddr};
use std::time::Duration;

use linux_embedded_hal::spidev::SpiModeFlags;

use futures::prelude::*;
use tokio::prelude::*;

use daemon_engine::{TcpConnection};
use daemon_engine::codecs::json::{JsonCodec};
use rr_mux::{Mux as BaseMux, Connector};

use crate::common::*;
use crate::error::Error;

pub mod spi;
use spi::Spi;
pub mod i2c;
use i2c::I2c;
pub mod pin;
use pin::Pin;

type Mux = BaseMux<u64, (), Request, Response, Error, ()>;


pub const TIMEOUT: Duration = Duration::from_secs(3);

///
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

    /// Connect to a new Spi instance
    pub fn spi(&mut self, path: &str, baud: u32, mode: SpiModeFlags) -> Result<Spi, Error> {
        debug!("attempting connection to SPI device: {}", path);
        let resp = self.mux.do_request(path, RequestKind::SpiConnect(SpiConnect{baud, mode: mode.bits()})).wait()?;
        match resp {
            ResponseKind::Ok => Ok(Spi::new(path.to_owned(), self.mux.clone())),
             _ => Err(Error::InvalidResponse(resp)),
        }
    }

    /// Connect to a new Pin instance
    pub fn pin(&mut self, path: &str) -> Result<Pin, Error> {
        debug!("attempting connection to Pin: {}", path);
        let resp = self.mux.do_request(path, RequestKind::PinConnect).wait()?;
        match resp {
            ResponseKind::Ok => Ok(Pin::new(path.to_owned(), self.mux.clone())),
             _ => Err(Error::InvalidResponse(resp)),
        }
    }

    /// Connect to a new I2c instance
    pub fn i2c(&mut self, path: &str) -> Result<I2c, Error> {
        debug!("attempting connection to I2c: {}", path);
        let resp = self.mux.do_request(path, RequestKind::I2cConnect).wait()?;
        match resp {
            ResponseKind::Ok => Ok(I2c::new(path.to_owned(), self.mux.clone())),
             _ => Err(Error::InvalidResponse(resp)),
        }
    }
}
