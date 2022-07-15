use crate::*;
use husky_compile_time::HuskyCompileTimeConfig;
use husky_eval_time::HuskyEvalTimeConfig;
use husky_feature_eval::EvaluatorConfig;
use husky_linkage_table::LinkageTableConfig;
use serde::{Deserialize, Serialize};
use vm::VMConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuskyDebuggerConfig {
    pub package_dir: PathBuf,
    pub opt_sample_id: Option<SampleId>,
    pub verbose: bool,
    pub report_missing_linkage: bool,
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
            report_missing_linkage: flags.report_missing_linkage || flags.verbose,
        }
    }

    pub fn eval_time(&self) -> HuskyEvalTimeConfig {
        HuskyEvalTimeConfig {
            evaluator: EvaluatorConfig {
                vm: VMConfig {
                    verbose: self.verbose,
                },
            },
            compile_time: HuskyCompileTimeConfig {
                __root_defn_resolver: todo!(),
                linkage_table: LinkageTableConfig {
                    report_missing_linkage: self.report_missing_linkage,
                },
            },
        }
    }
}
