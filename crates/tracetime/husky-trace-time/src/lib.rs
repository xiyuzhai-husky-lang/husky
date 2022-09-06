#![feature(try_trait_v2)]
mod impl_figure_canvas;
mod impl_figure_control;
mod impl_lines;
mod impl_ops;
mod impl_pin;
mod impl_restriction;
mod impl_subtraces;
mod impl_trace;
mod impl_trace_stalk;
mod impl_trace_stats;
mod monad;
mod trace_node;

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
use husky_print_utils::p;
use husky_runtime::*;
use husky_text::{HuskyText, TextQueryGroup};
use husky_trace::*;
use husky_trace_protocol::*;
use husky_vm::*;
use std::collections::HashMap;
use std::sync::Arc;
use trace_node::*;
use upcast::Upcast;
use vec_like::VecSet;

pub struct HuskyTracetime {
    runtime: HuskyRuntime,
    restriction: Restriction,
    pins: VecSet<TraceId>,
    trace_nodes: Vec<Option<TraceNode>>,
    opt_active_trace_id: Option<TraceId>,
    figure_canvases: VecSet<FigureCanvasKey>,
    figure_controls: HashMap<FigureControlKey, FigureControlData>,
    trace_stalks: HashMap<TraceStalkKey, TraceStalk>,
    trace_statss: HashMap<TraceStatsKey, Option<TraceStats>>,
    root_trace_ids: Vec<TraceId>,
    subtrace_ids_map: HashMap<SubtracesKey, Vec<TraceId>>,
}

impl HuskyTracetime {
    pub fn new(
        init_compile_time: impl FnOnce(&mut HuskyComptime),
        eval_time_config: HuskyRuntimeConfig,
    ) -> Self {
        let mut trace_time = Self {
            runtime: HuskyRuntime::new(init_compile_time, eval_time_config),
            trace_nodes: Default::default(),
            figure_canvases: Default::default(),
            figure_controls: Default::default(),
            trace_stalks: Default::default(),
            trace_statss: Default::default(),
            opt_active_trace_id: Default::default(),
            subtrace_ids_map: Default::default(),
            root_trace_ids: Default::default(),
            restriction: Default::default(),
            pins: Default::default(),
        };
        assert!(trace_time.restriction.opt_sample_id().is_none());
        trace_time.update();
        trace_time
    }

    pub fn opt_active_trace_id(&mut self) -> Option<TraceId> {
        self.opt_active_trace_id
    }

    pub fn activate(
        &mut self,
        trace_id: TraceId,
    ) -> __VMResult<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.opt_active_trace_id = Some(trace_id);
        Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
        ))
    }

    pub fn root_traces(&self) -> Vec<TraceId> {
        self.root_trace_ids.clone()
    }

    pub fn runtime(&self) -> &HuskyRuntime {
        &self.runtime
    }

    pub fn comptime(&self) -> &HuskyComptime {
        self.runtime.comptime()
    }

    pub fn all_trace_nodes(&self) -> Vec<TraceNodeData> {
        self.trace_nodes
            .iter()
            .filter_map(|opt_trace| opt_trace.as_ref().map(|node| node.to_data()))
            .collect()
    }

    pub fn subtrace_ids(&mut self, trace_id: TraceId) -> Vec<TraceId> {
        let trace = &self.trace(trace_id);
        let opt_sample_id = self.restriction.opt_sample_id();
        if !trace.raw_data.has_subtraces(opt_sample_id.is_some()) {
            return vec![];
        }
        let key = SubtracesKey::new(trace.raw_data.kind, trace_id, opt_sample_id);
        if let Some(subtrace_ids) = self.subtrace_ids_map.get(&key) {
            subtrace_ids.clone()
        } else {
            if let Some(subtrace_ids) = self.gen_subtraces(trace_id) {
                self.subtrace_ids_map
                    .insert(key.clone(), subtrace_ids.clone());
                subtrace_ids
            } else {
                todo!()
            }
        }
    }

    fn trace_node_data(&self, trace_id: TraceId) -> TraceNodeData {
        self.trace_nodes[trace_id.0].as_ref().unwrap().to_data()
    }

    pub(crate) fn next_id(&mut self) -> TraceId {
        self.trace_nodes.push(None);
        TraceId(self.trace_nodes.len() - 1)
    }

    fn new_trace(
        &mut self,
        opt_parent_id: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant<'static>,
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
        assert!(self.trace_nodes[trace.id().0].is_none());
        self.trace_nodes[trace_id.0] = Some(TraceNode {
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
    ) -> __VMResult<
        Option<(
            Vec<TraceNodeData>,
            Vec<TraceId>,
            Vec<(TraceStalkKey, TraceStalk)>,
            Vec<(TraceStatsKey, Option<TraceStats>)>,
        )>,
    > {
        let old_len = self.trace_nodes.len();
        let expansion = &mut self.trace_nodes[trace_id.0].as_mut().unwrap().expansion;
        *expansion = !*expansion;
        let subtrace_ids = self.subtrace_ids(trace_id);
        Ok(if self.trace_nodes.len() > old_len {
            let new_traces: Vec<TraceNodeData> = self.trace_nodes[old_len..]
                .iter()
                .map(|opt_node| opt_node.as_ref().unwrap().to_data())
                .collect();
            let trace_stalks = self.update_trace_stalks();
            let trace_stats = self.update_trace_statss()?;
            Some((new_traces, subtrace_ids, trace_stalks, trace_stats))
        } else {
            None
        })
    }

    pub fn is_expanded(&mut self, trace_id: TraceId) -> bool {
        self.trace_nodes[trace_id.0].as_ref().unwrap().expansion
    }

    pub fn toggle_show(&mut self, trace_id: TraceId) {
        let shown = &mut self.trace_nodes[trace_id.0].as_mut().unwrap().shown;
        *shown = !*shown
    }

    pub fn trace(&self, trace_id: TraceId) -> &Trace {
        &self.trace_nodes[trace_id.0].as_ref().unwrap().trace
    }

    pub(crate) unsafe fn trace_ref<'a>(&self, trace_id: TraceId) -> &'a Trace {
        let ptr: *const Trace = &self.trace_nodes[trace_id.0].as_ref().unwrap().trace;
        &*ptr
    }

    pub fn init_data(&mut self) -> InitData {
        let root_trace_ids = self.root_trace_ids.clone();
        // clear figure cache to reduce data transmission
        self.figure_canvases.clear();
        self.figure_controls.clear();
        let figure_canvases = self.update_figure_canvases().expect("todo");
        let figure_controls = self.update_figure_controls().expect("todo");
        let pins = self.pins.clone();
        let traces = self.all_trace_nodes();
        InitData {
            trace_init_data: TraceInitData {
                opt_active_trace_id: self.opt_active_trace_id,
                trace_nodes: traces,
                subtrace_ids_map: self
                    .subtrace_ids_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_stalks: self
                    .trace_stalks
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_statss: self
                    .trace_statss
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                root_trace_ids,
            },
            restriction: self.restriction.clone(),
            figure_canvases,
            figure_controls,
            pins,
        }
    }

    fn vm_config(&self) -> &VMConfig {
        self.runtime().vm_config()
    }
}
