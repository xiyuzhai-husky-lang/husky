#![allow(warnings)]
mod error;
mod query;
mod session;
mod tests;

use dataset::LabeledData;
pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};
use query::EvalFeature;
pub use query::{AskCompileTime, RuntimeQueryGroup, RuntimeQueryGroupStorage};

use common::{msg_once, HashMap};
use file::{FilePtr, FileQueryGroup};
use husky_lang_compile_time::*;
use stdx::sync::ARwLock;
use text::{RawTextQueryGroup, TextQueryGroupStorage};

use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use session::Session;
use trace::{AllocateTrace, FigureProps, Trace, TraceAllocator, TraceId, TraceKind, TraceStalk};
use vm::{run, AnyValueDyn, Instruction};

#[salsa::database(RuntimeQueryGroupStorage, TextQueryGroupStorage)]
pub struct HuskyLangRuntime {
    storage: salsa::Storage<HuskyLangRuntime>,
    compile_time: HuskyLangCompileTime,
    trace_allocator: TraceAllocator,
    session: Arc<Mutex<Session<'static>>>,
}

impl AskCompileTime for HuskyLangRuntime {
    fn compile_time(&self, _version: usize) -> &HuskyLangCompileTime {
        &self.compile_time
    }
}

impl RawTextQueryGroup for HuskyLangRuntime {
    fn raw_text(&self, file: FilePtr) -> Option<Arc<String>> {
        self.compile_time.raw_text(file)
    }
}

impl AllocateTrace for HuskyLangRuntime {
    fn trace_allocator(&self) -> &trace::TraceAllocator {
        &self.trace_allocator
    }
}

impl EvalFeature for HuskyLangRuntime {
    fn session(&self) -> &Arc<Mutex<Session<'static>>> {
        &self.session
    }
}

impl HuskyLangRuntime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        assert!(all_main_files.len() == 1);
        let current_package_main = all_main_files[0];
        let package = compile_time.package(current_package_main).unwrap();
        let mut runtime = Self {
            storage: Default::default(),
            compile_time,
            trace_allocator: Default::default(),
            session: Arc::new(Mutex::new(Session::new(&package).unwrap())),
        };
        runtime.set_version(0);
        runtime.set_package_main(current_package_main);
        runtime
    }

    pub async fn change_text(&mut self) {
        self.set_version(self.version() + 1);
    }

    pub fn figure(&self, id: TraceId) -> Option<FigureProps> {
        msg_once!("todo: figure");
        None
    }

    pub fn activate(&self, id: TraceId) {}

    pub fn toggle_expansion(&self, id: TraceId) {}

    pub fn toggle_show(&self, id: TraceId) {}

    pub fn lock_input(&self, input_temp: String) -> (Option<Option<usize>>, Option<String>) {
        if input_temp.len() == 0 {
            return (Some(None), None);
        }
        match input_temp.parse::<usize>() {
            Ok(idx) => (Some(Some(idx)), None),
            Err(e) => (None, Some(format!("lock input failed due to error: {}", e))),
        }
    }
}

impl salsa::Database for HuskyLangRuntime {}
