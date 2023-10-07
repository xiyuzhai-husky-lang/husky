// use crate::*;
// use husky_dev_comptime::ComptimeConfig;
// use husky_dev_runtime::RuntimeConfig;
// use husky_eval::EvaluatorConfig;
// use husky_linkage_table::LinkageTableConfig;
// use husky_root_static_defn::__resolve_root_defn;
// use husky_vm::VMConfig;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HuskyDeveloperConfig {
//     pub package_dir: PathBuf,
//     pub opt_sample_id: Option<SampleId>,
//     pub verbose: bool,
//     pub compiled: bool,
// }

// impl HuskyDeveloperConfig {
//     pub fn runtime(&self) -> RuntimeConfig {
//         RuntimeConfig {
//             evaluator: EvaluatorConfig {
//                 vm: VMConfig {
//                     verbose: self.verbose,
//                 },
//             },
//             comptime: ComptimeConfig {
//                 package_dir: self.package_dir.clone(),
//                 __resolve_root_defn,
//                 linkage_table: LinkageTableConfig {
//                     warn_missing_linkage: self.compiled,
//                 },
//             },
//         }
//     }
// }
