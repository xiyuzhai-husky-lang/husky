#![allow(warnings)]
mod error;
mod impl_figure;
mod query;
mod session;
mod tests;

use datasets::LabeledData;
pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};
use focus::Focus;
use json_result::JsonResult;
use query::EvalFeature;
pub use query::{RuntimeQueryGroup, RuntimeQueryGroupStorage};

use check_utils::*;
use compile_time_db::*;
use file::{FilePtr, FileQueryGroup};
use print_utils::*;
use session::Session;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use text::{RawTextQueryGroup, TextQueryGroupStorage};
use trace::{CreateTrace, FigureProps, Trace, TraceFactory, TraceId, TraceKind, TraceStalk};
use visual_runtime::*;
use vm::{AnyValueDyn, Instruction};

#[salsa::database(
    VisualQueryGroupStorage,
    RuntimeQueryGroupStorage,
    TextQueryGroupStorage
)]
pub struct HuskyLangRuntime {
    storage: salsa::Storage<HuskyLangRuntime>,
    compile_time: HuskyLangCompileTime,
    traces: Arc<TraceFactory<'static>>,
    session: Arc<Mutex<Session<'static>>>,
    focus: Focus,
    expansions: HashMap<TraceId, bool>,
    showns: HashMap<TraceId, bool>,
    pack_main: FilePtr,
}

impl AskCompileTime for HuskyLangRuntime {
    fn compile_time(&self, _version: usize) -> &HuskyLangCompileTime {
        msg_once!("check version");
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

    fn feature_query_group(&self) -> &dyn FeatureQueryGroup {
        &self.compile_time
    }
}

impl HuskyLangRuntime {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        should_eq!(all_main_files.len(), 1);
        let pack_main = all_main_files[0];
        let pack = match compile_time.pack(pack_main) {
            Ok(pack) => pack,
            Err(error) => {
                println!("{:?}", error);
                panic!()
            }
        };
        let mut runtime = Self {
            storage: Default::default(),
            session: Arc::new(Mutex::new(Session::new(&pack, &compile_time).unwrap())),
            compile_time,
            traces: Default::default(),
            focus: Default::default(),
            expansions: Default::default(),
            showns: Default::default(),
            pack_main,
        };
        runtime.set_version(0);
        runtime.set_pack_main(pack_main);
        runtime
    }

    pub fn traces(&self) -> &TraceFactory<'static> {
        &self.traces
    }

    pub async fn change_text(&mut self) {
        self.set_version(self.version() + 1);
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

    pub fn focus(&self) -> &Focus {
        &self.focus
    }

    pub fn decode_focus(&self, command: &str) -> JsonResult<Focus> {
        if command.len() == 0 {
            return Ok(Focus::default());
        }
        match command.parse::<usize>() {
            Ok(id) => Ok(Focus {
                opt_input_id: Some(id),
            }),
            Err(e) => Err(format!("lock input failed due to error: {}", e)),
        }
    }

    pub fn lock_input(&mut self, command: &str) -> (Option<Option<usize>>, Option<String>) {
        if command.len() == 0 {
            return (Some(None), None);
        }
        match command.parse::<usize>() {
            Ok(id) => {
                self.focus = Focus {
                    opt_input_id: Some(id),
                };
                (Some(Some(id)), None)
            }
            Err(e) => (None, Some(format!("lock input failed due to error: {}", e))),
        }
    }
}

impl salsa::Database for HuskyLangRuntime {}
