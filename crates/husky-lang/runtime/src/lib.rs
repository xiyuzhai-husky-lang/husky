#![allow(warnings)]
mod error;
mod query;
mod session;
mod tests;

use dataset::LabeledData;
pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};
use query::EvalFeature;
pub use query::{AskCompileTime, RuntimeQueryGroup, RuntimeQueryGroupStorage};

use common::{msg_once, p, should_eq, HashMap};
use file::{FilePtr, FileQueryGroup};
use husky_lang_compile_time::*;
use stdx::sync::ARwLock;
use text::{RawTextQueryGroup, TextQueryGroupStorage};

use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use session::Session;
use trace::{CreateTrace, FigureProps, Trace, TraceFactory, TraceId, TraceKind, TraceStalk};
use vm::{AnyValueDyn, Instruction};

#[salsa::database(RuntimeQueryGroupStorage, TextQueryGroupStorage)]
pub struct HuskyLangRuntime {
    storage: salsa::Storage<HuskyLangRuntime>,
    compile_time: HuskyLangCompileTime,
    traces: Arc<TraceFactory<'static>>,
    session: Arc<Mutex<Session<'static>>>,
    opt_input_id: Option<usize>,
    expansions: HashMap<TraceId, bool>,
    showns: HashMap<TraceId, bool>,
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

impl CreateTrace<'static> for HuskyLangRuntime {
    fn trace_factory(&self) -> &trace::TraceFactory<'static> {
        &self.traces
    }
    fn trace_factory_arc(&self) -> Arc<trace::TraceFactory<'static>> {
        self.traces.clone()
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
        should_eq!(all_main_files.len(), 1);
        let current_package_main = all_main_files[0];
        let package = match compile_time.package(current_package_main) {
            Ok(package) => package,
            Err(error) => {
                println!("{}", error.message);
                panic!()
            }
        };
        let mut runtime = Self {
            storage: Default::default(),
            compile_time,
            traces: Default::default(),
            session: Arc::new(Mutex::new(Session::new(&package).unwrap())),
            opt_input_id: None,
            expansions: Default::default(),
            showns: Default::default(),
        };
        runtime.set_version(0);
        runtime.set_package_main(current_package_main);
        runtime
    }

    pub fn traces(&self) -> &TraceFactory<'static> {
        &self.traces
    }

    pub async fn change_text(&mut self) {
        self.set_version(self.version() + 1);
    }

    pub fn figure(&self, id: TraceId) -> Option<FigureProps> {
        msg_once!("todo: figure");
        None
    }

    pub fn toggle_expansion(&mut self, id: TraceId) {
        let expanded = self.expansions.entry(id).or_insert(false);
        *expanded = !*expanded;
    }

    pub fn is_expanded(&mut self, trace: &Trace) -> bool {
        *self.expansions.entry(trace.id()).or_insert(false)
    }

    pub fn expansions(&mut self) -> HashMap<TraceId, bool> {
        self.expansions.clone()
    }

    pub fn toggle_show(&mut self, id: TraceId) {
        let shown = self.showns.entry(id).or_insert(false);
        *shown = !*shown;
    }

    pub fn showns(&self) -> HashMap<TraceId, bool> {
        self.showns.clone()
    }

    pub fn opt_input_id(&self) -> Option<usize> {
        self.opt_input_id
    }

    pub fn lock_input(&mut self, input_id_str: &str) -> (Option<Option<usize>>, Option<String>) {
        if input_id_str.len() == 0 {
            return (Some(None), None);
        }
        match input_id_str.parse::<usize>() {
            Ok(id) => {
                self.opt_input_id = Some(id);
                (Some(Some(id)), None)
            }
            Err(e) => (None, Some(format!("lock input failed due to error: {}", e))),
        }
    }
}

impl salsa::Database for HuskyLangRuntime {}
