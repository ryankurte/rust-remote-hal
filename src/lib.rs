
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate structopt;
extern crate futures;
extern crate tokio;
extern crate rand;
extern crate futures_timer;
extern crate embedded_hal;
#[macro_use]
extern crate log;
extern crate hex;
extern crate try_from;

extern crate daemon_engine;
extern crate rr_mux;

pub mod common;
pub mod manager;
pub mod error;
pub mod server;
pub mod local;
pub mod remote;



use std::net::{SocketAddr, ToSocketAddrs};

/// Remote address helper
/// Fetches a SocketAddr from the REMOTE_HAL_SERVER environmental variable
pub fn remote_addr() -> SocketAddr {
    let a = std::env::var("REMOTE_HAL_SERVER").expect("REMOTE_HAL_SERVER environmental variable undefined (and feature remote enabled)");

    let mut a = a.to_socket_addrs().expect("Error parsing socket address");

    match a.next() {
        Some(a) => a,
        None => panic!("No socket addresses found"),
    }
}
