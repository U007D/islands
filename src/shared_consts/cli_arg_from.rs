use super::*;

/// `const from()`/`into()` `CliArgs` converter.  Because `const` trait impls are not yet stable as
/// of the time of this writing, this impl is conditionally compiled out until a stable
/// `const_trait_impl` feature lands in `rustc` at which time the following will be used (per parent
/// module's `cfg` attributes)..
impl const From<CliArg> for usize {
    #[must_use]
    fn from(cli_arg: CliArg) -> Self {
        #[allow(clippy::enum_glob_use)]
        use CliArg::*;
        match cli_arg {
            NoProgramNameArg => 0,
            HasProgramNameArg => 1,
        }
    }
}
