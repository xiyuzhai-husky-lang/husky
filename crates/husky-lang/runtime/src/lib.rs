#![allow(warnings)]
mod error;
mod session;
mod tests;

use common::HashMap;
use file::{FilePtr, FileQuery};
use husky_lang_compile_time::*;
use tokio::sync::Mutex;

use std::{borrow::Cow, sync::Arc};

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};

use session::Session;
use trace::{FigureProps, Trace, TraceKind};
use vm::{run, AnyValueDyn, Instruction};

#[derive(Debug)]
pub struct HuskyLangRuntime {
    compile_time: HuskyLangCompileTime,
    all_main_files: Vec<FilePtr>,
    current_package_main: Option<FilePtr>,
    session: Option<Session<'static>>,
    traces: HashMap<usize, Arc<Trace>>,
    root_traces: Vec<Arc<Trace>>,
}

impl HuskyLangRuntime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        let current_package_main = if all_main_files.len() == 1 {
            Some(all_main_files[0])
        } else {
            None
        };
        let mut runtime = Self {
            compile_time,
            all_main_files,
            current_package_main,
            session: None,
            traces: Default::default(),
            root_traces: vec![],
        };
        runtime.init_root_traces();
        runtime
    }

    fn init_root_traces(&mut self) {
        self.root_traces = if let Some(current_package_main) = self.current_package_main {
            let main_feature_block = self
                .compile_time
                .main_feature_block(current_package_main)
                .unwrap();
            self.store_traces(vec![Trace::main(current_package_main, main_feature_block)])
        } else {
            vec![]
        };
    }

    pub async fn change_text(&mut self) {
        self.all_main_files = self.compile_time.all_main_files();
        self.current_package_main = if self.all_main_files.len() == 1 {
            Some(self.all_main_files[0])
        } else {
            None
        };
        todo!("send updates on all main files and currrent package main")
    }

    pub fn root_traces(&self) -> Vec<Arc<Trace>> {
        self.root_traces.clone()
    }

    fn store_traces(&mut self, traces: Vec<Arc<Trace>>) -> Vec<Arc<Trace>> {
        traces
            .into_iter()
            .map(|trace| {
                self.traces.insert(trace.id, trace.clone());
                trace
            })
            .collect()
    }

    pub fn subtraces(&self, id: usize) -> Vec<Arc<Trace>> {
        let trace = &self.traces[&id];
        match trace.kind {
            TraceKind::Mock { ref tokens } => trace::mock::subtraces(id),
            TraceKind::Main {
                main_file,
                ref feature_block,
            } => trace::eval_block_traces(Some(trace.id), feature_block),
            TraceKind::Stmt(_) => todo!(),
            TraceKind::Expr(_) => todo!(),
        }
    }

    pub fn figure(&self, id: usize) -> Option<FigureProps> {
        trace::mock::figure(id)
    }

    pub fn activate(&self, id: usize) {}

    pub fn toggle_expansion(&self, id: usize) {}
}
