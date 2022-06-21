use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn keyed_trace_stalk(&mut self, trace_id: TraceId) -> (TraceStalkKey, TraceStalkRawData) {
        let sample_id = self.attention.opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_raw_data(sample_id, &self.trace(trace_id).raw_data);
        if !self.trace_stalks.contains_key(&key) {
            self.trace_stalks
                .insert(key.clone(), self.produce_trace_stalk(trace_id, sample_id));
        }
        let trace_stalk_raw_data = self.trace_stalks[&key].clone();
        (key, trace_stalk_raw_data)
    }

    fn produce_trace_stalk(&self, trace_id: TraceId, input_id: usize) -> TraceStalkRawData {
        let trace: &Trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(ref block) => TraceStalkRawData {
                extra_tokens: vec![
                    husky_tracer_protocol::fade!(" = "),
                    self.runtime.eval_feature_repr(block, input_id).into(),
                ],
            },
            TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
                FeatureStmtVariant::Init { varname, ref value } => TraceStalkRawData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(value, input_id).into(),
                    ],
                },
                FeatureStmtVariant::Assert { ref condition } => TraceStalkRawData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(condition, input_id).into(),
                    ],
                },
                FeatureStmtVariant::Return { ref result } => TraceStalkRawData {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(result, input_id).into(),
                    ],
                },
                FeatureStmtVariant::ConditionFlow { ref branches } => panic!(),
                FeatureStmtVariant::ReturnXml { ref result } => todo!(),
            },
            TraceVariant::FeatureBranch(_) => TraceStalkRawData {
                extra_tokens: vec![],
            },
            TraceVariant::FeatureExpr(ref expr) => TraceStalkRawData {
                extra_tokens: vec![
                    husky_tracer_protocol::fade!(" = "),
                    self.runtime.eval_feature_expr(expr, input_id).into(),
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
                vm::ControlSnapshot::None => TraceStalkRawData::default(),
                vm::ControlSnapshot::Return(_) => todo!(),
                vm::ControlSnapshot::Break => todo!(),
                vm::ControlSnapshot::Err(_) => todo!(),
            },
            TraceVariant::ProcBranch { .. } => panic!(),
        }
    }

    pub fn collect_new_trace_stalks(&mut self) -> Vec<(TraceStalkKey, TraceStalkRawData)> {
        let mut trace_stalks = Vec::new();
        for root_trace_id in self.root_trace_ids.clone() {
            self.collect_new_trace_stalks_within_trace(root_trace_id, &mut trace_stalks);
        }
        trace_stalks
    }

    fn collect_new_trace_stalks_within_trace(
        &mut self,
        trace_id: TraceId,
        trace_stalks: &mut Vec<(TraceStalkKey, TraceStalkRawData)>,
    ) {
        let sample_id = self.attention.opt_sample_id().unwrap();
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.raw_data;
        let trace_stalk_key = TraceStalkKey::from_trace_raw_data(sample_id, trace_raw_data);
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.trace_stalks.contains_key(&trace_stalk_key) {
            trace_stalks.push(self.keyed_trace_stalk(trace_id))
        }
        for associated_trace_id in associated_trace_ids {
            self.collect_new_trace_stalks_within_trace(associated_trace_id, trace_stalks)
        }
        if expanded {
            for subtrace_id in self.subtrace_ids(trace_id) {
                self.collect_new_trace_stalks_within_trace(subtrace_id, trace_stalks)
            }
        }
    }
}
