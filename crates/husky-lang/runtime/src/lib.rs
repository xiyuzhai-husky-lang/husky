#![allow(warnings)]
mod error;
mod query;
mod session;
mod tests;

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};
pub use query::{AskCompileTime, RuntimeQueryGroup, RuntimeQueryGroupStorage};

use common::HashMap;
use file::{FilePtr, FileQuery};
use husky_lang_compile_time::*;
use tokio::sync::Mutex;

use std::{borrow::Cow, sync::Arc};

use session::Session;
use trace::{AllocateTrace, FigureProps, Trace, TraceAllocator, TraceKind};
use vm::{run, AnyValueDyn, Instruction};

#[salsa::database(RuntimeQueryGroupStorage)]
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

    pub fn figure(&self, id: usize) -> Option<FigureProps> {
        trace::mock::figure(id)
    }

    pub fn activate(&self, id: usize) {}

    pub fn toggle_expansion(&self, id: usize) {}
}

impl salsa::Database for HuskyLangRuntime {}
