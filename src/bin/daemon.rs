
use std::net::SocketAddr;

use structopt::StructOpt;

extern crate tokio;
use tokio::prelude::*;
use tokio::runtime::Runtime;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::{TermLogger, LevelFilter};

extern crate remote_hal;
use remote_hal::server::Server;

#[derive(StructOpt)]
#[structopt(name = "Remote HAL CLI", about = "A Command Line Interface (CLI) for interacting with a remote-hal server")]
pub struct Options {
    #[structopt(short = "b", long = "bind-address", default_value = "0.0.0.0:10004")]
    /// Specify the bind address of the remote-hal server
    bind_addr: SocketAddr,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

fn main() {
    // Fetch arguments
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(opts.level, simplelog::Config::default()).unwrap();

    let mut rt = Runtime::new().unwrap();

    let handle = futures::lazy(move || {
        info!("starting remote-hal server (bound to: {})", opts.bind_addr);

        let _server = Server::new(opts.bind_addr);

        info!("remote-hal server running!");

        Ok(())
    });

    rt.spawn(handle);

    rt.shutdown_on_idle()
    .wait().unwrap();
}