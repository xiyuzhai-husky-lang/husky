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
use husky_dev_runtime::*;
use husky_eager_semantics::*;
use husky_entity_semantics::*;
use husky_feature_eval::EvalFeature;
use husky_feature_gen::*;
use husky_file::FilePtr;
use husky_init_syntax::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_print_utils::{msg_once, p};
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

pub struct HuskyDevtime {
    runtime: HuskyDevRuntime,
    state: HuskyDevtimeState,
}

impl HuskyDevtime {
    pub fn new(runtime_config: RuntimeConfig) -> Self {
        let mut devtime = Self {
            runtime: HuskyDevRuntime::new(runtime_config),
            state: Default::default(),
        };
        assert!(devtime.state.presentation.opt_sample_id().is_none());
        devtime.hot_reload();
        devtime
    }

    pub fn opt_active_trace_id(&self) -> Option<TraceId> {
        self.state.presentation.opt_active_trace_id()
    }

    pub fn activate_trace(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDevtimeTakeChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.activate_trace(trace_id);
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        let change = self.take_change()?;
        HuskyDevtimeTakeChangeM::Ok((
            change.figure_canvases.opt_new_entries().unwrap_or_default(),
            change.figure_controls.opt_new_entries().unwrap_or_default(),
        ))
    }

    pub fn root_traces(&self) -> Vec<TraceId> {
        self.state
            .root_traces()
            .iter()
            .map(|id| id.unwrap())
            .collect()
    }

    pub fn runtime(&self) -> &HuskyDevRuntime {
        &self.runtime
    }

    pub fn all_trace_nodes(&self) -> Vec<TraceNodeData> {
        self.state
            .trace_nodes
            .iter()
            .map(|node| node.to_data())
            .collect()
    }

    // move this to somewhere proper
    pub(crate) fn update_subtraces(&mut self, trace_id: TraceId) -> HuskyDevtimeUpdateM<()> {
        let trace = &self.trace(trace_id);
        let opt_sample_id = self.state.presentation.opt_sample_id();
        if !trace.raw_data.has_subtraces(opt_sample_id.is_some()) {
            return HuskyDevtimeUpdateM::Ok(());
        }
        let key = SubtracesKey::new(trace.raw_data.kind, trace_id, opt_sample_id);
        if self.state.subtrace_ids_map.get(&key).is_none() {
            if let Some(subtraces) = self.gen_subtraces(trace_id) {
                self.state
                    .subtrace_ids_map
                    .insert_new(key.clone(), subtraces.clone());
            } else {
                todo!()
            }
        }
        HuskyDevtimeUpdateM::Ok(())
    }

    pub(crate) fn subtraces(&self, trace_id: TraceId) -> Vec<TraceId> {
        let trace = &self.trace(trace_id);
        let opt_sample_id = self.state.presentation.opt_sample_id();
        if !trace.raw_data.has_subtraces(opt_sample_id.is_some()) {
            return vec![];
        }
        let key = SubtracesKey::new(trace.raw_data.kind, trace_id, opt_sample_id);
        self.state
            .subtrace_ids_map
            .get(&key)
            .as_ref()
            .unwrap()
            .to_vec()
    }

    fn trace_node_data(&self, trace_id: TraceId) -> TraceNodeData {
        self.state.trace_nodes[trace_id.raw()].to_data()
    }

    pub(crate) fn next_id(&mut self) -> TraceId {
        self.state.trace_nodes.push(TraceNode::Uninitialized);
        TraceId::new(self.state.trace_nodes.len() - 1)
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
                    can_have_subtraces,
                    reachable,
                    lines,
                    kind: variant.kind(),
                    opt_arrival_indicator: variant
                        .opt_arrival_indicator()
                        .map(|ind| ind.feature.id()),
                    // opt_stats: variant.opt_stats(self.runtime()).expect("todo"),
                },
                variant,
                file,
                range,
            }
        };
        assert!(!self.state.trace_nodes[trace.id().raw()].initialized());
        self.state.trace_nodes.set_elem(
            trace_id.raw(),
            TraceNode::Initialized {
                expanded: false,
                shown: match trace.raw_data.kind {
                    TraceKind::FeatureExprLazy
                    | TraceKind::FeatureExprEager
                    | TraceKind::EagerExpr => trace.raw_data.opt_parent_id.is_some(),
                    _ => true,
                },
                trace,
            },
        );
        trace_id
    }

    pub fn toggle_expansion(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDevtimeTakeChangeM<
        Option<(
            Vec<TraceNodeData>,
            Vec<TraceId>,
            Vec<(TraceStalkKey, TraceStalk)>,
            Vec<(TraceStatsKey, Option<TraceStats>)>,
        )>,
    > {
        self.state
            .trace_nodes
            .apply_update_elem(trace_id.raw(), |node| node.toggle_expansion())?;
        self.update_subtraces(trace_id); // ad hoc
        self.update()?;
        let change = self.take_change()?;
        HuskyDevtimeTakeChangeM::Ok(
            if let Some(new_trace_nodes) = change.trace_nodes.opt_new_entries() {
                let mut subtraces = change.subtrace_ids_map.opt_new_entries().unwrap();
                assert_eq!(subtraces.len(), 1);
                let (_, subtraces) = subtraces.pop().unwrap();
                Some((
                    new_trace_nodes,
                    subtraces,
                    change.trace_stalks.opt_new_entries().unwrap_or_default(),
                    change.trace_statss.opt_new_entries().unwrap_or_default(),
                ))
            } else {
                None
            },
        )
    }

    pub fn expanded(&mut self, trace_id: TraceId) -> bool {
        self.state.trace_nodes[trace_id.raw()].expanded()
    }

    pub fn toggle_show(&mut self, trace_id: TraceId) -> HuskyDevtimeTakeChangeM<()> {
        self.state
            .trace_nodes
            .apply_update_elem(trace_id.raw(), |node| node.toggle_shown())?;
        HuskyDevtimeTakeChangeM::Ok(())
    }

    pub fn trace(&self, trace_id: TraceId) -> &Trace {
        self.state.trace_nodes[trace_id.raw()].trace()
    }

    pub(crate) unsafe fn trace_ref<'a>(&self, trace_id: TraceId) -> &'a Trace {
        let ptr: *const Trace = self.state.trace_nodes[trace_id.raw()].trace();
        &*ptr
    }

    fn vm_config(&self) -> &VMConfig {
        self.runtime().vm_config()
    }
}
