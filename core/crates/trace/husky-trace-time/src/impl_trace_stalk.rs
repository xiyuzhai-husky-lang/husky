use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_text::HuskyText;
use husky_word::RootIdentifier;
use vm::{ControlSnapshot, History, VMControl};

impl HuskyTraceTime {
    pub fn keyed_trace_stalk(&mut self, trace_id: TraceId) -> (TraceStalkKey, TraceStalkData) {
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_data(sample_id, &self.trace(trace_id).raw_data);
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
            TraceVariant::Main(ref repr) => self.trace_stalk_from_result(
                self.runtime().eval_feature_repr(repr, sample_id),
                repr.ty(),
            ),
            TraceVariant::EntityFeature { ref repr, .. } => self.trace_stalk_from_result(
                self.runtime().eval_feature_repr(repr, sample_id),
                repr.ty(),
            ),
            TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
                FeatureLazyStmtVariant::Init { varname, ref value } => {
                    self.trace_stalk_from_expr(value, sample_id)
                }
                FeatureLazyStmtVariant::Assert { ref condition } => {
                    self.trace_stalk_from_expr(condition, sample_id)
                }
                FeatureLazyStmtVariant::Return { ref result } => {
                    self.trace_stalk_from_expr(result, sample_id)
                }
                FeatureLazyStmtVariant::ConditionFlow { ref branches } => panic!(),
                FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
            },
            TraceVariant::FeatureBranch(_) => TraceStalkData {
                extra_tokens: vec![],
            },
            TraceVariant::FeatureExpr(ref expr) => self.trace_stalk_from_expr(expr, sample_id),
            TraceVariant::FeatureCallArgument {
                name: ident,
                ref argument,
            } => self.trace_stalk_from_expr(argument, sample_id),
            TraceVariant::Module { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. }
            | TraceVariant::FuncBranch { .. }
            | TraceVariant::ProcBranch { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::EagerCallArgument { .. } => TraceStalkData::default(),
        }
    }

    pub fn collect_new_trace_stalks(&mut self) -> Vec<(TraceStalkKey, TraceStalkData)> {
        if let Some(sample_id) = self.restriction.opt_sample_id() {
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
        let trace_stalk_key = TraceStalkKey::from_trace_data(sample_id, trace_raw_data);
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

    fn trace_stalk_from_expr(&self, expr: &FeatureExpr, sample_id: SampleId) -> TraceStalkData {
        self.trace_stalk_from_result(
            self.runtime().eval_feature_expr(expr, sample_id),
            expr.expr.ty(),
        )
    }

    fn trace_stalk_from_result(
        &self,
        result: __VMResult<__Register<'static>>,
        ty: EntityRoutePtr,
    ) -> TraceStalkData {
        TraceStalkData {
            extra_tokens: vec![fade!(" = "), self.trace_token_from_result(result, ty)],
        }
    }

    pub(crate) fn trace_token_from_result(
        &self,
        result: __VMResult<__Register<'static>>,
        ty: EntityRoutePtr,
    ) -> TraceTokenData {
        match result {
            Ok(value) => self.trace_token_from_value(value, ty),
            Err(e) => {
                todo!()
                // e.into()
            }
        }
    }

    pub(crate) fn trace_token_from_value(
        &self,
        value: __Register<'static>,
        ty: EntityRoutePtr,
    ) -> TraceTokenData {
        TraceTokenData {
            kind: TraceTokenKind::Fade,
            value: self.comptime().print_short(&value, ty),
            opt_associated_trace_id: None,
        }
    }
}
