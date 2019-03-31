
use std::io;

use crate::common::ResponseKind;
use daemon_engine::DaemonError;
use serde_json::{Error as JsonError};
use tokio::timer::timeout::Error as TimeoutError;

#[derive(Debug)]
pub enum Error {
    Io(io::ErrorKind),
    Json(JsonError),
    Timeout,
    Remote(String),
    Daemon(DaemonError),
    InvalidResponse(ResponseKind),
    None(()),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e.kind())
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Json(e)
    }
}

impl From<DaemonError> for Error {
    fn from(e: DaemonError) -> Self {
        Error::Daemon(e)
    }
}

impl From<()> for Error {
    fn from(e: ()) -> Self {
        Error::None(e)
    }
}

impl From<TimeoutError<Error>> for Error {
    fn from(e: TimeoutError<Error>) -> Self {
        if e.is_inner() {
            e.into_inner().unwrap()
        } else {
            Error::Timeout
        }
    }
}