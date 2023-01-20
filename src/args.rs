/// See `args::unit_tests` for sample usages and how to drive CLI argument parsing from tests.
#[cfg(test)]
mod unit_tests;

use bool_ext::BoolExt;
use he_std::args::ParseArgs;

use crate::error::arg::Error;

/// A data structure representing the user's supplied command-line arguments given to the program.
/// It is constructed via a [`TryFrom<Box<[String]>>`] constructor, representing a possible series
/// of [`String`]s derived from the command-line arguments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    pub sample_arg: String,
}

impl Args {
    /// Manual count of number of fields in `Args` (== number of arguments expected)
    const EXPECTED_ARG_COUNT: usize = 1;
}

/// Fallible constructor
impl TryFrom<Box<[String]>> for Args {
    type Error = Error;

    fn try_from(args: Box<[String]>) -> std::result::Result<Self, Self::Error> {
        let arg_count = args.len();
        (arg_count == Self::EXPECTED_ARG_COUNT).ok_or_err_with(
            || Error::BadArgCount(arg_count, Self::EXPECTED_ARG_COUNT),
            || Self {
                sample_arg: args.into_vec().remove(0),
            },
        )
    }
}

/// Attach default parsing methods for converting from [`OsString`] to [`String`] with selectable
/// UTF-8 conversion strategies.
impl ParseArgs for Args {
    type Error = Error;
}
