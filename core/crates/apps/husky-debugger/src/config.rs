use crate::*;
use husky_compile_time::HuskyCompileTimeConfig;
use husky_eval_time::HuskyEvalTimeConfig;
use husky_feature_eval::EvaluatorConfig;
use husky_linkage_table::LinkageTableConfig;
use husky_root_static_defn::__resolve_root_defn;
use serde::{Deserialize, Serialize};
use vm::VMConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuskyDebuggerConfig {
    pub package_dir: PathBuf,
    pub opt_sample_id: Option<SampleId>,
    pub verbose: bool,
    pub compiled: bool,
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
            compiled: flags.compiled,
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
                package_dir: self.package_dir.clone(),
                __resolve_root_defn,
                linkage_table: LinkageTableConfig {
                    warn_missing_linkage: self.compiled,
                },
            },
        }
    }
}
