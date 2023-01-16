mod wrapped_std_io;

use crate::shared_consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{} {:?}", msg::ERR_FILE_CREATE, 0)]
    FileCreateError(wrapped_std_io::Error),
    #[error("{} {:?}", msg::ERR_FILE_OPEN, 0)]
    FileOpenError(wrapped_std_io::Error),
    #[error("{} {:?}", msg::ERR_FILE_READ, 0)]
    FileReadError(wrapped_std_io::Error),
    #[error("{} {:?}", msg::ERR_FILE_WRITE, 0)]
    FileWriteError(wrapped_std_io::Error),
    #[error(transparent)]
    IoError(#[from] wrapped_std_io::Error),
}

impl Error {
    pub fn as_file_create_err_cx<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileCreateError(error.into())
    }

    pub fn as_file_open_err_cx<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileOpenError(error.into())
    }

    pub fn as_file_read_err_cx<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileReadError(error.into())
    }

    pub fn as_file_write_err_cx<TError>(error: TError) -> Self where TError: Into<wrapped_std_io::Error> {
        Self::FileWriteError(error.into())
    }
}