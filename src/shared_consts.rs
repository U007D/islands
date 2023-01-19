// This conditional compilation must be in a sub-module because code within a `cfg` block must parse
// and `impl const From...` will not parse until the `const_trait_impl` stabilizes.  Rust skips
// parsing of conditionally compiled out submodules.
#[cfg(feature = "const_trait_impl")]
mod cli_arg_from;
#[cfg(not(feature = "const_trait_impl"))]
mod cli_arg_into;
#[cfg(feature = "const_trait_impl")]
mod cli_arg_from;

pub mod msg;

/// Typically the first argument passed into an application is the executable name.  According to
/// [std::env::args] documentation, this first argument may be arbitrary text, or may even not exist
/// at all!  [`CliArg`] provides context on the meaning of the first argument of the program.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CliArg {
    NoProgramNameArg,
    HasProgramNameArg,
}
