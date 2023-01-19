pub mod arg;
pub mod io;
pub mod non_empty_rect_list_2d;

use crate::shared_consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Crate-level error type grouping all error categories emitted by this crate under one umbrella.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ARG, 0)]
    ArgError(#[from] arg::Error),
    #[error("{}: {:?}: ", msg::ERR_IO, 0)]
    IoError(#[from] io::Error),
}
