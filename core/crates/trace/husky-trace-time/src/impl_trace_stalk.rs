use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn trace_stalk(&mut self, trace_id: TraceId, input_id: usize) -> &TraceStalk {
        let key = TraceStalkKey::new(&self.trace(trace_id).props, input_id);
        if !self.trace_stalks.contains_key(&key) {
            self.trace_stalks
                .insert(key.clone(), self.produce_trace_stalk(trace_id, input_id));
        }
        &self.trace_stalks[&key]
    }

    fn produce_trace_stalk(&self, trace_id: TraceId, input_id: usize) -> TraceStalk {
        let trace: &Trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(ref block) => TraceStalk {
                extra_tokens: vec![
                    husky_tracer_protocol::fade!(" = "),
                    self.runtime.eval_feature_repr(block, input_id).into(),
                ],
            },
            TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
                FeatureStmtVariant::Init { varname, ref value } => TraceStalk {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(value, input_id).into(),
                    ],
                },
                FeatureStmtVariant::Assert { ref condition } => TraceStalk {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(condition, input_id).into(),
                    ],
                },
                FeatureStmtVariant::Return { ref result } => TraceStalk {
                    extra_tokens: vec![
                        husky_tracer_protocol::fade!(" = "),
                        self.runtime.eval_feature_expr(result, input_id).into(),
                    ],
                },
                FeatureStmtVariant::ConditionFlow { ref branches } => panic!(),
                FeatureStmtVariant::ReturnXml { ref result } => todo!(),
            },
            TraceVariant::FeatureBranch(_) => TraceStalk {
                extra_tokens: vec![],
            },
            TraceVariant::FeatureExpr(ref expr) => TraceStalk {
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
                vm::ControlSnapshot::None => TraceStalk::default(),
                vm::ControlSnapshot::Return(_) => todo!(),
                vm::ControlSnapshot::Break => todo!(),
                vm::ControlSnapshot::Err(_) => todo!(),
            },
            TraceVariant::ProcBranch { .. } => panic!(),
        }
    }
}
