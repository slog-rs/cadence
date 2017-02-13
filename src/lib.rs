//!
//!
//!

extern crate cadence;
extern crate slog;

use std::error;
use std::fmt;
use std::sync::Arc;
use cadence::{MetricClient, MetricError};
use slog::{Drain, Record, OwnedKeyValueList};


///
///
///
#[derive(Clone)]
pub struct CadenceDrain {
    client: Arc<MetricClient + Sync + Send>,
}

impl CadenceDrain {
    ///
    ///
    ///
    pub fn new<T>(client: T) -> CadenceDrain where T: MetricClient + Sync + Send + 'static {
        CadenceDrain { client: Arc::new(client) }
    }
}


impl Drain for CadenceDrain {
    type Error = ::Error;

    fn log(&self, info: &Record, _: &OwnedKeyValueList) -> Result<(), Error> {
        Ok(())
    }
}


///
///
///
#[derive(Debug)]
pub enum Error {
    ClientError(MetricError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::ClientError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ClientError(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::ClientError(ref e) => e.description()
        }
    }
}
