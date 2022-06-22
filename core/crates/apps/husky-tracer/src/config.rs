use crate::*;

#[derive(Debug)]
pub(crate) struct DebuggerConfig {
    pub(crate) verbose: bool,
    pub(crate) opt_sample_idx: Option<String>,
}

impl DebuggerConfig {
    pub(crate) fn from_env() -> Self {
        match flags::HuskyTracerCommand::from_env() {
            Ok(flags) => Self {
                verbose: flags.verbose,
                opt_sample_idx: flags.sample_idx,
            },
            Err(_) => Self {
                verbose: false,
                opt_sample_idx: None,
            },
        }
    }
}
