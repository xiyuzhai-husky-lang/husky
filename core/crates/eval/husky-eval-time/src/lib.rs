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

use check_utils::*;
use husky_compile_time::*;
use husky_datasets_protocol::LabeledData;
use husky_feature_eval::*;
use husky_feature_eval::{EvalFeature, Session};
use husky_file::{FilePtr, FileQueryGroup};
use husky_text::TextQueryGroupStorage;
use husky_trace_protocol::*;
use indexmap::IndexMap;
use json_map::JsonListMap;
use json_result::JsonResult;
use print_utils::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use variant::*;
use vm::{Instruction, VMConfig, __AnyValueDyn};

#[salsa::database(
    husky_feature_gen::FeatureGenQueryGroupStorage,
    husky_instruction_gen::InstructionGenQueryGroupStorage
)]
pub struct HuskyEvalTime {
    storage: salsa::Storage<HuskyEvalTime>,
    compile_time: HuskyCompileTime,
    compile_time_version: usize,
    feature_interner: husky_feature_gen::FeatureInterner,
    variant: HuskyEvalTimeVariant,
    package_main: FilePtr,
    config: HuskyEvalTimeConfig,
}

pub struct HuskyEvalTimeConfig {
    pub evaluator: EvaluatorConfig,
    pub compile_time: HuskyCompileTimeConfig,
}

impl HuskyEvalTime {
    pub fn new(
        // __root_defn: fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
        init_compile_time: impl FnOnce(&mut HuskyCompileTime),
        config: HuskyEvalTimeConfig,
    ) -> HuskyEvalTimeSingletonKeeper {
        let mut compile_time = HuskyCompileTime::new(config.compile_time.clone());
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        should_eq!(all_main_files.len(), 1);
        let package_main = all_main_files[0];
        let feature_interner = husky_feature_gen::new_feature_interner();
        let mut eval_time = Self {
            storage: Default::default(),
            variant: HuskyEvalTimeVariant::None,
            compile_time,
            compile_time_version: 0,
            package_main,
            config,
            feature_interner,
        };
        eval_time.init();
        eval_time.into()
    }

    fn init(&mut self) {
        let compile_time = &self.compile_time;
        let all_diagnostics = compile_time.all_diagnostics();
        if all_diagnostics.len() > 0 {
            p!(all_diagnostics);
            panic!("diagnostic errors")
        }
        let package = match compile_time.package(self.package_main) {
            Ok(package) => package,
            Err(error) => {
                compile_time.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        self.variant = HuskyEvalTimeVariant::Learning {
            session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
        }
    }
}
