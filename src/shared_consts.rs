// This conditional compilation must be in a sub-module because code within a `cfg` block must parse
// and `impl const From...` will not parse until the `const_trait_impl` stabilizes.  Rust skips
// parsing of conditionally compiled out submodules.
#[cfg(feature = "const_trait_impl")]
mod cli_arg_from;
#[cfg(not(feature = "const_trait_impl"))]
mod cli_arg_into;

pub mod msg;

pub const EXPECTED_ARG_COUNT: usize = 2;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CliArg {
    NoProgramNameArg,
    HasProgramNameArg,
}
