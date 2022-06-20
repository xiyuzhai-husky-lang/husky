use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn trace_stalk_with_key(
        &mut self,
        trace_id: TraceId,
        input_id: usize,
    ) -> (TraceStalkKey, TraceStalkRawData) {
        let key = TraceStalkKey::from_trace_raw_data(input_id, &self.trace(trace_id).raw_data);
        if !self.trace_stalks.contains_key(&key) {
            self.trace_stalks
                .insert(key.clone(), self.produce_trace_stalk(trace_id, input_id));
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
}
