mod impl_figure;
mod impl_necessary;
mod query;
mod tests;

use datasets::LabeledData;
use eval_feature::{EvalFeature, Session};
use focus::Focus;
pub use impl_figure::*;
use indexmap::IndexMap;
use json_map::JsonListMap;
use json_result::JsonResult;
pub use query::*;

use check_utils::*;
use compile_time_db::*;
use eval_feature::*;
use file::{FilePtr, FileQueryGroup};
use print_utils::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use text::TextQueryGroupStorage;
use trace::*;
use visual_runtime::*;
use vm::{AnyValueDyn, Instruction};

#[salsa::database(RuntimeVisualizerQueryGroupStorage, TraceQueryGroupStorage)]
pub struct HuskyRuntime {
    storage: salsa::Storage<HuskyRuntime>,
    compile_time: HuskyCompileTime,
    compile_time_version: usize,
    traces: TraceFactory<'static>,
    session: Arc<Mutex<Session<'static>>>,
    focus: Focus,
    expansions: HashMap<TraceId, bool>,
    showns: HashMap<TraceId, bool>,
    figure_controls: HashMap<String, FigureControlProps>,
    package_main: FilePtr,
    config: HuskyRuntimeConfig,
}

pub struct HuskyRuntimeConfig {
    verbose: bool,
}

impl HuskyRuntime {
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
            storage: Default::default(),
            session: Arc::new(Mutex::new(
                Session::new(&pack, &compile_time, verbose).unwrap(),
            )),
            compile_time,
            compile_time_version: 0,
            traces: Default::default(),
            focus: Default::default(),
            expansions: Default::default(),
            showns: Default::default(),
            package_main,
            figure_controls: Default::default(),
            config: HuskyRuntimeConfig { verbose },
        };
        runtime.set_version(0);
        runtime.set_pack_main(package_main);
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

    pub fn expansions(&self) -> &HashMap<TraceId, bool> {
        &self.expansions
    }

    pub fn toggle_show(&mut self, id: TraceId) {
        let shown = self.showns.entry(id).or_insert(false);
        *shown = !*shown;
    }

    pub fn showns(&self) -> &HashMap<TraceId, bool> {
        &self.showns
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

impl salsa::Database for HuskyRuntime {}
