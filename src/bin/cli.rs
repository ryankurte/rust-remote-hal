
use std::net::ToSocketAddrs;

use structopt::StructOpt;

extern crate tokio;
use tokio::prelude::*;
use tokio::runtime::Runtime;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::{TermLogger, LevelFilter};

extern crate remote_hal;
use remote_hal::remote::Client;
use remote_hal::common::RequestKind;


#[derive(StructOpt)]
#[structopt(name = "Remote HAL CLI", about = "A Command Line Interface (CLI) for interacting with a remote-hal server")]
pub struct Options {
    #[structopt(short = "s", long = "remote-server", default_value = "127.0.0.1:10004")]
    /// Specify the hostname of the remote-hal server
    hostname: String,

    /// Remote device for target subcommand
    device: String,

    #[structopt(subcommand)]
    /// Request for remote-hal server
    command: RequestKind,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

fn main() {
    // Load options
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(opts.level, simplelog::Config::default()).unwrap();

    let mut addrs = opts.hostname.to_socket_addrs().expect("could not parse socket addresses");
    let addr = addrs.next().expect("no socket address found");

    let command = opts.command;
    let device = opts.device;

    info!("connecting to remote-hal server: {:?}", &addr);
    debug!("device: {:?}", device);
    debug!("command: {:?}", command);

    let mut rt = Runtime::new().unwrap();

    // Create client
    let handle = Client::new(addr)
    .map_err(|e| {
        error!("error connecting to remote-hal server: {:?}", e);
        std::process::exit(-1);
    })
    .and_then(move |mut c| {
        info!("connected, sending request: {:?}", command);
        c.request(&device, command)
    }).map_err(|e| {
        error!("error sending command to remote-hal server: {:?}", e);
        std::process::exit(-2);
    }).map(|resp| {
        println!("resp: {:#?}", resp);
    });

    rt.block_on(handle.map(|_| () ).map_err(|e| panic!(e) )).unwrap();
}

