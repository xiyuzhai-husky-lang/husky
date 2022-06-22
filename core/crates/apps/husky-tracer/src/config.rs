use crate::*;

#[derive(Debug)]
pub(crate) struct DebuggerConfig {
    pub(crate) verbose: bool,
    pub(crate) opt_sample_id: Option<String>,
}

impl DebuggerConfig {
    pub(crate) fn from_env() -> Self {
        match flags::HuskyTracerCommand::from_env() {
            Ok(flags) => Self {
                verbose: flags.verbose,
                opt_sample_id: flags.sample_id,
            },
            Err(_) => Self {
                verbose: false,
                opt_sample_id: None,
            },
        }
    }
}
