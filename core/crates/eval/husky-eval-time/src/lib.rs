mod impl_necessary;
mod query;
mod singleton;
mod variant;

pub use feature_gen::{AllocateUniqueFeature, FeatureGenQueryGroup, FeatureGenQueryGroupStorage};
pub use instruction_gen::InstructionGenQueryGroup;
pub use query::*;
pub use singleton::*;
pub use visualizer_gen::VisualizerQueryGroup;

use check_utils::*;
use datasets_static_defn::LabeledData;
use eval_feature::*;
use eval_feature::{EvalFeature, Session};
use file::{FilePtr, FileQueryGroup};
use husky_compile_time::*;
use husky_tracer_protocol::*;
use indexmap::IndexMap;
use json_map::JsonListMap;
use json_result::JsonResult;
use print_utils::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use text::TextQueryGroupStorage;
use variant::*;
use vm::{AnyValueDyn, Instruction};

#[salsa::database(
    feature_gen::FeatureGenQueryGroupStorage,
    instruction_gen::InstructionGenQueryGroupStorage,
    visualizer_gen::VisualizerQueryGroupStorage
)]
pub struct HuskyEvalTime {
    storage: salsa::Storage<HuskyEvalTime>,
    compile_time: HuskyCompileTime,
    compile_time_version: usize,
    feature_interner: feature_gen::FeatureInterner,
    variant: HuskyEvalTimeVariant,
    package_main: FilePtr,
    config: HuskyEvalTimeConfig,
}

pub struct HuskyEvalTimeConfig {
    verbose: bool,
}

impl HuskyEvalTime {
    pub fn new(
        static_root_defn: fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
        init_compile_time: impl FnOnce(&mut HuskyCompileTime),
        verbose: bool,
    ) -> HuskyEvalTimeSingleton {
        unsafe {
            HUSKY_EVAL_TIME_SINGLETON = None;
        }
        let mut compile_time = HuskyCompileTime::new(static_root_defn);
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        should_eq!(all_main_files.len(), 1);
        let package_main = all_main_files[0];
        let feature_interner = feature_gen::new_feature_interner();
        let mut eval_time = Self {
            storage: Default::default(),
            variant: HuskyEvalTimeVariant::None,
            compile_time,
            compile_time_version: 0,
            package_main,
            config: HuskyEvalTimeConfig { verbose },
            feature_interner,
        };
        eval_time.init();
        eval_time.into()
    }

    fn init(&mut self) {
        let compile_time = &self.compile_time;
        for module in compile_time.all_modules() {
            let diagnostics_reserve = compile_time.diagnostics_reserve(module);
            if diagnostics_reserve.len() > 0 {
                p!(diagnostics_reserve.data());
                panic!("diagnostic errors")
            }
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
            session: Session::new(&package, self, self.verbose()).unwrap(),
        }
    }
}
