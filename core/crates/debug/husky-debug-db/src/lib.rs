mod impl_figure;
mod impl_figure_control;
mod impl_necessary;

use compile_time_db::HuskyCompileTime;
use husky_debugger_protocol::*;
use impl_figure::*;
use json_map::JsonListMap;
use print_utils::*;
use std::{collections::HashMap, sync::Arc};
use trace::*;

#[salsa::database(TraceQueryGroupStorage)]
pub struct HuskyDebugTime {
    storage: salsa::Storage<HuskyDebugTime>,
    pub(crate) active_trace_id: Option<TraceId>,
    pub(crate) subtraces_map: HashMap<(TraceId, Option<usize>), Vec<TraceId>>,
    pub(crate) focus: Focus,
    pub(crate) figure_controls: HashMap<FigureControlKey, FigureControlProps>,
}

impl salsa::Database for HuskyDebugTime {}

impl HuskyDebugTime {
    pub fn new(
        init_compile_time: impl FnOnce(&mut HuskyCompileTime),
        verbose: bool,
    ) -> HuskyDebugTime {
        Self {
            storage: Default::default(),
            active_trace_id: todo!(),
            subtraces_map: todo!(),
            focus: todo!(),
            figure_controls: todo!(),
        }
    }

    pub fn set_subtraces(
        &mut self,
        parent: &Trace,
        effective_opt_input_id: Option<usize>,
        // effective_opt_input_id_for_subtraces: Option<usize>,
        subtraces: &[Arc<Trace>],
    ) {
        assert!(self
            .subtraces_map
            .insert(
                (parent.id(), effective_opt_input_id),
                subtraces.iter().map(|trace| trace.id()).collect()
            )
            .is_none())
    }

    pub fn focus(&self) -> Focus {
        self.focus.clone()
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
