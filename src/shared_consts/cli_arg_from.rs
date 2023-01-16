use super::*;

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
