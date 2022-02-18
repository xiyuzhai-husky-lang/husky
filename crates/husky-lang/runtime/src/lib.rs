#![allow(warnings)]
mod error;
mod query;
mod session;
mod tests;

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};
pub use query::{AskCompileTime, RuntimeQueryGroup, RuntimeQueryGroupStorage};

use common::{msg_once, HashMap};
use file::{FilePtr, FileQueryGroup};
use husky_lang_compile_time::*;
use stdx::sync::ARwLock;
use text::{RawTextQueryGroup, TextQueryGroupStorage};
use tokio::sync::Mutex;

use std::{borrow::Cow, sync::Arc};

use session::Session;
use trace::{AllocateTrace, FigureProps, Trace, TraceAllocator, TraceId, TraceKind, TraceStalk};
use vm::{run, AnyValueDyn, Instruction};

#[salsa::database(RuntimeQueryGroupStorage, TextQueryGroupStorage)]
pub struct HuskyLangRuntime {
    storage: salsa::Storage<HuskyLangRuntime>,
    compile_time: HuskyLangCompileTime,
    trace_allocator: TraceAllocator,
    session: Option<Session<'static>>,
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

impl HuskyLangRuntime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time(&mut compile_time);
        let mut runtime = Self {
            storage: Default::default(),
            compile_time,
            trace_allocator: Default::default(),
            session: None,
        };
        runtime.init_salsa();
        runtime
    }

    fn init_salsa(&mut self) {
        self.set_version(0);
        let all_main_files = self.compile_time.all_main_files();
        assert!(all_main_files.len() == 1);
        let current_package_main = all_main_files[0];
        self.set_package_main(current_package_main);
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

    pub fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk> {
        todo!()
    }
}

impl salsa::Database for HuskyLangRuntime {}
