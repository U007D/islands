use std::fmt::Debug;

use thiserror::Error;

use crate::shared_consts::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// [`NonEmptyRectList2D`] errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}.", msg::ERR_RECT_NO_INPUT_DATA)]
    NoData(String),
    #[error("{}.", msg::ERR_RECT_NON_RECTANGULAR_INPUT)]
    NonRectangularInput,
    #[error("{}. {}: ({}, {})", msg::ERR_ROW_COLUMN_OVERFLOW, msg::ROWS_COLUMNS, 0, 1)]
    TooManyElements(usize, usize),
}