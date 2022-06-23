mod impl_necessary;
mod query;
mod tests;
mod variant;

pub use query::*;

use check_utils::*;
use datasets::LabeledData;
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
use vm::{AnyValueDyn, Instruction};

pub struct HuskyEvalTime {
    compile_time: HuskyCompileTime,
    compile_time_version: usize,
    session: Session<'static>,
    package_main: FilePtr,
    config: HuskyEvalTimeConfig,
}

pub struct HuskyEvalTimeConfig {
    verbose: bool,
}

impl HuskyEvalTime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyCompileTime), verbose: bool) -> Self {
        let mut compile_time = HuskyCompileTime::default();
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        should_eq!(all_main_files.len(), 1);
        for module in compile_time.all_modules() {
            let diagnostics_reserve = compile_time.diagnostics_reserve(module);
            if diagnostics_reserve.len() > 0 {
                p!(diagnostics_reserve.data());
                panic!("diagnostic errors")
            }
        }
        let package_main = all_main_files[0];
        let pack = match compile_time.package(package_main) {
            Ok(pack) => pack,
            Err(error) => {
                compile_time.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        let mut runtime = Self {
            session: Session::new(&pack, &compile_time, verbose).unwrap(),
            compile_time,
            compile_time_version: 0,
            package_main,
            config: HuskyEvalTimeConfig { verbose },
        };
        runtime
    }
}
