//!
//!
//!

extern crate cadence;
extern crate slog;

use std::sync::Arc;
use cadence::MetricClient;

///
///
///
pub struct CadenceDrain {
    client: Arc<MetricClient + Sync + Send>,
}

pub struct DrainBuilder<'a> {
    prefix: &'a str,
}
