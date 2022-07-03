use crate::*;
use husky_eval_time::HuskyEvalTimeConfig;
use vm::VMConfig;

#[derive(Debug)]
pub(crate) struct HuskyDebuggerConfig {
    pub(crate) opt_sample_id: Option<String>,
    pub(crate) verbose: bool,
    pub(crate) report_vm: bool,
}

impl HuskyDebuggerConfig {
    pub(crate) fn from_env() -> Self {
        match flags::HuskyDebuggerCommand::from_env() {
            Ok(flags) => Self {
                opt_sample_id: flags.sample_id,
                verbose: flags.verbose,
                report_vm: flags.report_vm || flags.verbose,
            },
            Err(_) => panic!(),
            // Self {
            //     verbose: false,
            //     opt_sample_id: None,
            // },
        }
    }

    pub fn eval_time(&self) -> HuskyEvalTimeConfig {
        HuskyEvalTimeConfig {
            vm_config: VMConfig {
                verbose: self.verbose,
            },
        }
    }
}
