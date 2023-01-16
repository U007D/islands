use super::*;

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
