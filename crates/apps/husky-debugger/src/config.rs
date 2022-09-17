use crate::*;
use husky_comptime::HuskyComptimeConfig;
use husky_feature_eval::EvaluatorConfig;
use husky_linkage_table::LinkageTableConfig;
use husky_root_static_defn::__resolve_root_defn;
use husky_runtime::HuskyRuntimeConfig;
use husky_vm::VMConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuskyDebuggerConfig {
    pub package_dir: PathBuf,
    pub opt_sample_id: Option<SampleId>,
    pub verbose: bool,
    pub compiled: bool,
}

impl HuskyDebuggerConfig {
    pub fn eval_time(&self) -> HuskyRuntimeConfig {
        HuskyRuntimeConfig {
            evaluator: EvaluatorConfig {
                vm: VMConfig {
                    verbose: self.verbose,
                },
            },
            comptime: HuskyComptimeConfig {
                package_dir: self.package_dir.clone(),
                __resolve_root_defn,
                linkage_table: LinkageTableConfig {
                    warn_missing_linkage: self.compiled,
                },
            },
        }
    }
}
