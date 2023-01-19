use super::*;

/// Temporary `const into()` converter.  Because `const` trait impls are not yet stable as of the
/// time of this writing, the following `const into()` impl is provided.  It is conditionally
/// compiled from its parent module such that a stable `const From<CliArg>` impl will be used
/// instead once it stabilizes.
impl CliArg {
    #[must_use]
    pub const fn into(self) -> usize {
        #[allow(clippy::enum_glob_use)]
        use CliArg::*;
        match self {
            NoProgramNameArg => 0,
            HasProgramNameArg => 1,
        }
    }
}
