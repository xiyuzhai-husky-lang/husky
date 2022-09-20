#![feature(try_trait_v2)]
mod db;
mod impl_figure_canvas;
mod impl_figure_control;
mod impl_lines;
mod impl_pin;
mod impl_trace_stalk;
mod impl_trace_stats;
mod ops;
mod restriction;
mod state;
mod subtraces;
mod trace;
mod trace_node;

pub use ops::*;
pub use state::*;

use husky_comptime::*;
use husky_defn_head::Parameter;
use husky_eager_semantics::*;
use husky_entity_semantics::*;
use husky_feature_eval::EvalFeature;
use husky_feature_gen::*;
use husky_file::FilePtr;
use husky_init_syntax::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_print_utils::{msg_once, p};
use husky_runtime::*;
use husky_text::{HuskyText, TextQueryGroup};
use husky_trace::*;
use husky_trace_protocol::*;
use husky_vm::*;
use monad::Monad;
use std::collections::HashMap;
use std::sync::Arc;
use trace_node::*;
use upcast::Upcast;
use vec_like::VecSet;

pub struct Debugtime {
    runtime: HuskyRuntime,
    state: DebugtimeState,
}

impl Debugtime {
    pub fn new(
        init_runtime: impl FnOnce(&mut HuskyRuntime),
        eval_time_config: HuskyRuntimeConfig,
    ) -> Self {
        let mut debugtime = Self {
            runtime: HuskyRuntime::new(init_runtime, eval_time_config),
            state: Default::default(),
        };
        assert!(debugtime.state.restriction.opt_sample_id().is_none());
        debugtime.update();
        debugtime
    }

    pub fn opt_active_trace_id(&self) -> Option<TraceId> {
        *self.state.opt_active_trace_id
    }

    pub fn activate(
        &mut self,
        trace_id: TraceId,
    ) -> DebugtimeStageChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.opt_active_trace_id.set(Some(trace_id));
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        DebugtimeStageChangeM::Ok((todo!(), todo!()))
    }

    pub fn root_traces(&self) -> Vec<TraceId> {
        self.state.root_traces().to_vec()
    }

    pub fn runtime(&self) -> &HuskyRuntime {
        &self.runtime
    }

    pub fn comptime(&self) -> &HuskyComptime {
        self.runtime.comptime()
    }

    pub fn all_trace_nodes(&self) -> Vec<TraceNodeData> {
        self.state
            .trace_nodes
            .iter()
            .filter_map(|opt_trace| opt_trace.as_ref().map(|node| node.to_data()))
            .collect()
    }

    pub fn subtrace_ids(&mut self, trace_id: TraceId) -> Vec<TraceId> {
        let trace = &self.trace(trace_id);
        let opt_sample_id = self.state.restriction.opt_sample_id();
        if !trace.raw_data.has_subtraces(opt_sample_id.is_some()) {
            return vec![];
        }
        let key = SubtracesKey::new(trace.raw_data.kind, trace_id, opt_sample_id);
        if let Some(subtrace_ids) = self.state.subtrace_ids_map.get(&key) {
            subtrace_ids.clone()
        } else {
            if let Some(subtrace_ids) = self.gen_subtraces(trace_id) {
                self.state
                    .subtrace_ids_map
                    .insert(key.clone(), subtrace_ids.clone());
                subtrace_ids
            } else {
                todo!()
            }
        }
    }

    fn trace_node_data(&self, trace_id: TraceId) -> TraceNodeData {
        self.state.trace_nodes[trace_id.0]
            .as_ref()
            .unwrap()
            .to_data()
    }

    pub(crate) fn next_id(&mut self) -> TraceId {
        self.state.trace_nodes.push(None);
        TraceId(self.state.trace_nodes.len() - 1)
    }

    fn new_trace(
        &mut self,
        opt_parent_id: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant,
    ) -> TraceId {
        let trace_id = self.next_id();
        let trace = {
            let (file, range) = variant.file_and_range();
            let reachable = variant.reachable();
            let can_have_subtraces = variant.can_have_subtraces(reachable);
            let lines = self.trace_lines(indent, &variant, opt_parent_id.is_some());
            Trace {
                raw_data: TraceData {
                    id: trace_id,
                    opt_parent_id,
                    indent,
                    compile_time_version: 0, //compile time version
                    can_have_subtraces,
                    reachable,
                    lines,
                    kind: variant.kind(),
                    // opt_stats: variant.opt_stats(self.runtime()).expect("todo"),
                },
                variant,
                file,
                range,
            }
        };
        assert!(self.state.trace_nodes[trace.id().0].is_none());
        self.state.trace_nodes[trace_id.0] = Some(TraceNode {
            expansion: false,
            shown: match trace.raw_data.kind {
                TraceKind::FeatureExprLazy | TraceKind::FeatureExprEager | TraceKind::EagerExpr => {
                    trace.raw_data.opt_parent_id.is_some()
                }
                _ => true,
            },
            trace,
        });
        trace_id
    }

    pub fn toggle_expansion(
        &mut self,
        trace_id: TraceId,
    ) -> DebugtimeStageChangeM<
        Option<(
            Vec<TraceNodeData>,
            Vec<TraceId>,
            Vec<(TraceStalkKey, TraceStalk)>,
            Vec<(TraceStatsKey, Option<TraceStats>)>,
        )>,
    > {
        let old_len = self.state.trace_nodes.len();
        let expansion = &mut self.state.trace_nodes[trace_id.0]
            .as_mut()
            .unwrap()
            .expansion;
        *expansion = !*expansion;
        let subtrace_ids = self.subtrace_ids(trace_id);
        DebugtimeStageChangeM::Ok(if self.state.trace_nodes.len() > old_len {
            let new_traces: Vec<TraceNodeData> = self.state.trace_nodes[old_len..]
                .iter()
                .map(|opt_node| opt_node.as_ref().unwrap().to_data())
                .collect();
            self.update_trace_stalks();
            self.update_trace_statss()?;
            Some((todo!(), todo!(), todo!(), todo!()))
        } else {
            None
        })
    }

    pub fn is_expanded(&mut self, trace_id: TraceId) -> bool {
        self.state.trace_nodes[trace_id.0]
            .as_ref()
            .unwrap()
            .expansion
    }

    pub fn toggle_show(&mut self, trace_id: TraceId) {
        let shown = &mut self.state.trace_nodes[trace_id.0].as_mut().unwrap().shown;
        *shown = !*shown
    }

    pub fn trace(&self, trace_id: TraceId) -> &Trace {
        &self.state.trace_nodes[trace_id.0].as_ref().unwrap().trace
    }

    pub(crate) unsafe fn trace_ref<'a>(&self, trace_id: TraceId) -> &'a Trace {
        let ptr: *const Trace = &self.state.trace_nodes[trace_id.0].as_ref().unwrap().trace;
        &*ptr
    }

    fn vm_config(&self) -> &VMConfig {
        self.runtime().vm_config()
    }
}
