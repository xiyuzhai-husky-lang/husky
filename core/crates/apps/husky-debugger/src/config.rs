use crate::*;
use husky_eval_time::HuskyEvalTimeConfig;
use vm::VMConfig;

#[derive(Debug)]
pub struct HuskyDebuggerConfig {
    pub package_dir: PathBuf,
    pub opt_sample_id: Option<SampleId>,
    pub verbose: bool,
    pub report_vm: bool,
}

impl HuskyDebuggerConfig {
    pub(crate) fn from_env() -> Self {
        let flags = match flags::HuskyDebuggerFlags::from_env() {
            Ok(flags) => flags,
            Err(_) => panic!(),
        };
        let package_dir = flags.package_dir.unwrap();
        Self {
            package_dir,
            opt_sample_id: flags
                .sample_id
                .map(|text| SampleId(text.parse::<usize>().unwrap())),
            verbose: flags.verbose,
            report_vm: flags.report_vm || flags.verbose,
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
