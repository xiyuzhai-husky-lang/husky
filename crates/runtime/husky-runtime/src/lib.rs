mod impl_necessary;
mod impl_train;
mod query;
mod utils;
mod variant;

pub use husky_compile_time::*;
pub use husky_feature_gen::{
    AllocateUniqueFeature, FeatureGenQueryGroup, FeatureGenQueryGroupStorage,
};
pub use husky_instruction_gen::InstructionGenQueryGroup;
pub use query::*;
pub use utils::*;

use husky_check_utils::*;
use husky_compile_time::*;
use husky_datasets_interface::LabeledData;
use husky_feature_eval::*;
use husky_feature_eval::{EvalFeature, Session};
use husky_file::{FilePtr, FileQueryGroup};
use husky_print_utils::*;
use husky_text::TextQueryGroupStorage;
use husky_trace_protocol::*;
use indexmap::IndexMap;
use json_map::JsonListMap;
use json_result::JsonResult;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use variant::*;
use vm::{Instruction, VMConfig};

#[salsa::database(
    husky_feature_gen::FeatureGenQueryGroupStorage,
    husky_instruction_gen::InstructionGenQueryGroupStorage,
    husky_data_viewer::HuskyDataViewerQueryGroupStorage
)]
pub struct HuskyRuntime {
    storage: salsa::Storage<HuskyRuntime>,
    comptime: HuskyComptime,
    compile_time_version: usize,
    feature_interner: husky_feature_gen::FeatureInterner,
    variant: HuskyRuntimeVariant,
    config: HuskyRuntimeConfig,
}

#[derive(Debug)]
pub struct HuskyRuntimeConfig {
    pub evaluator: EvaluatorConfig,
    pub comptime: HuskyComptimeConfig,
}

impl HuskyRuntime {
    pub fn new(
        init_comptime: impl FnOnce(&mut HuskyComptime),
        config: HuskyRuntimeConfig,
    ) -> HuskyRuntime {
        let mut comptime = HuskyComptime::new(config.comptime.clone());
        init_comptime(&mut comptime);
        let all_main_files = comptime.all_target_entrances();
        should_eq!(all_main_files.len(), 1, "config = {config:?}");
        let target_entrance = all_main_files[0];
        let feature_interner = husky_feature_gen::new_feature_interner();
        let mut eval_time = Self {
            storage: Default::default(),
            variant: HuskyRuntimeVariant::None,
            comptime: comptime,
            compile_time_version: 0,
            config,
            feature_interner,
        };
        eval_time.init();
        eval_time.into()
    }

    fn init(&mut self) {
        let comptime = &self.comptime;
        let all_diagnostics = comptime.all_diagnostics();
        if all_diagnostics.len() > 0 {
            p!(all_diagnostics);
            panic!("diagnostic errors")
        }
        let package = match comptime.package(comptime.opt_target_entrance().unwrap()) {
            Ok(package) => package,
            Err(error) => {
                comptime.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        self.variant = HuskyRuntimeVariant::Learning {
            session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
        }
    }
}
