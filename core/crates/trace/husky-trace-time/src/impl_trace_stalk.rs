use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn keyed_trace_stalk(&mut self, trace_id: TraceId) -> (TraceStalkKey, TraceStalkData) {
        let sample_id = self.attention.opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_raw_data(sample_id, &self.trace(trace_id).raw_data);
        if !self.trace_stalks.contains_key(&key) {
            self.trace_stalks
                .insert(key.clone(), self.produce_trace_stalk(trace_id, sample_id));
        }
        let trace_stalk_raw_data = self.trace_stalks[&key].clone();
        (key, trace_stalk_raw_data)
    }

    fn produce_trace_stalk(&self, trace_id: TraceId, sample_id: SampleId) -> TraceStalkData {
        let trace: &Trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(ref block) => TraceStalkData {
                extra_tokens: vec![
                    husky_tracer_protocol::fade!(" = "),
                    self.eval_time_singleton
                        .eval_feature_repr(block, sample_id)
                        .into(),
                ],
            },
            TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
                FeatureStmtVariant::Init { varname, ref value } => TraceStalkData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.eval_time_singleton
                            .eval_feature_lazy_expr(value, sample_id)
                            .into(),
                    ],
                },
                FeatureStmtVariant::Assert { ref condition } => TraceStalkData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.eval_time_singleton
                            .eval_feature_lazy_expr(condition, sample_id)
                            .into(),
                    ],
                },
                FeatureStmtVariant::Return { ref result } => TraceStalkData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.eval_time_singleton
                            .eval_feature_lazy_expr(result, sample_id)
                            .into(),
                    ],
                },
                FeatureStmtVariant::ConditionFlow { ref branches } => panic!(),
                FeatureStmtVariant::ReturnXml { ref result } => todo!(),
            },
            TraceVariant::FeatureBranch(_) => TraceStalkData {
                extra_tokens: vec![],
            },
            TraceVariant::FeatureExpr(ref expr) => TraceStalkData {
                extra_tokens: vec![
                    husky_tracer_protocol::fade!(" = "),
                    self.eval_time_singleton
                        .eval_feature_lazy_expr(expr, sample_id)
                        .into(),
                ],
            },
            TraceVariant::FeatureCallInput { .. } => todo!(),
            TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. } => panic!(),
            TraceVariant::LoopFrame {
                loop_frame_data: ref vm_loop_frame,
                ..
            } => match vm_loop_frame.control {
                vm::ControlSnapshot::None => TraceStalkData::default(),
                vm::ControlSnapshot::Return(_) => todo!(),
                vm::ControlSnapshot::Break => todo!(),
                vm::ControlSnapshot::Err(_) => todo!(),
            },
            TraceVariant::ProcBranch { .. } => panic!(),
        }
    }

    pub fn collect_new_trace_stalks(&mut self) -> Vec<(TraceStalkKey, TraceStalkData)> {
        if let Some(sample_id) = self.attention.opt_sample_id() {
            let mut trace_stalks = Vec::new();
            for root_trace_id in self.root_trace_ids.clone() {
                self.collect_new_trace_stalks_within_trace(
                    sample_id,
                    root_trace_id,
                    &mut trace_stalks,
                );
            }
            trace_stalks
        } else {
            vec![]
        }
    }

    fn collect_new_trace_stalks_within_trace(
        &mut self,
        sample_id: SampleId,
        trace_id: TraceId,
        trace_stalks: &mut Vec<(TraceStalkKey, TraceStalkData)>,
    ) {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let trace_stalk_key = TraceStalkKey::from_trace_raw_data(sample_id, trace_raw_data);
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.trace_stalks.contains_key(&trace_stalk_key) {
            trace_stalks.push(self.keyed_trace_stalk(trace_id))
        }
        for associated_trace_id in associated_trace_ids {
            self.collect_new_trace_stalks_within_trace(sample_id, associated_trace_id, trace_stalks)
        }
        if expanded {
            for subtrace_id in self.subtrace_ids(trace_id) {
                self.collect_new_trace_stalks_within_trace(sample_id, subtrace_id, trace_stalks)
            }
        }
    }
}
