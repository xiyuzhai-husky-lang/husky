use crate::*;
use husky_ethereal_term::EtherealTerm;

impl Devtime {
    pub fn trace_stalk(&self, trace_id: TraceId) -> &TraceStalk {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_data(sample_id, &self.trace(trace_id).raw_data);
        &self.state.trace_stalks[&key]
    }

    pub(crate) fn gen_trace_stalk(&mut self, trace_id: TraceId) -> __VMResult<()> {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_data(sample_id, &self.trace(trace_id).raw_data);
        if !self.state.trace_stalks.contains(&key) {
            self.state
                .trace_stalks
                .insert_new(key, self.produce_trace_stalk(trace_id, sample_id));
        }
        Ok(())
    }

    fn produce_trace_stalk(&self, trace_id: TraceId, sample_id: SampleId) -> TraceStalk {
        let trace: &Trace = self.trace(trace_id);
        todo!()
        // match trace.variant {
        //     TraceVariant::Main(ref repr) => self.trace_stalk_from_result(
        //         self.runtime().eval_feature_repr_cached(repr, sample_id),
        //         repr.ty(),
        //     ),
        //     TraceVariant::EntityVal { ref repr, .. } => self.trace_stalk_from_result(
        //         self.runtime().eval_feature_repr_cached(repr, sample_id),
        //         repr.ty(),
        //     ),
        //     TraceVariant::ValStmt(ref stmt) => match stmt.variant {
        //         ValStmtData::Init { ref value, .. } => self.trace_stalk_from_expr(value, sample_id),
        //         ValStmtData::Assert { ref condition } => {
        //             self.trace_stalk_from_expr(condition, sample_id)
        //         }
        //         ValStmtData::Require { ref condition, .. } => {
        //             self.trace_stalk_from_expr(condition, sample_id)
        //         }
        //         ValStmtData::Return { ref result }
        //         | ValStmtData::ReturnUnveil { ref result, .. } => {
        //             self.trace_stalk_from_expr(result, sample_id)
        //         }
        //         ValStmtData::ConditionFlow { .. } => panic!(),
        //         ValStmtData::ReturnHtml { .. } => todo!(),
        //     },
        //     TraceVariant::ValBranch(_) => Default::default(),
        //     TraceVariant::LazyExpr(ref expr) => self.trace_stalk_from_expr(expr, sample_id),
        //     TraceVariant::ValCallArgument { ref argument, .. } => {
        //         self.trace_stalk_from_expr(argument, sample_id)
        //     }
        //     TraceVariant::Module { .. }
        //     | TraceVariant::FuncStmt { .. }
        //     | TraceVariant::EagerStmt { .. }
        //     | TraceVariant::EagerExpr { .. }
        //     | TraceVariant::CallHead { .. }
        //     | TraceVariant::FuncBranch { .. }
        //     | TraceVariant::EagerBranch { .. }
        //     | TraceVariant::LoopFrame { .. }
        //     | TraceVariant::EagerCallArgument { .. } => TraceStalk::default(),
        // }
    }

    pub(crate) fn update_trace_stalks(&mut self) -> __VMResult<()> {
        if let Some(sample_id) = self.state.presentation().opt_sample_id() {
            // ad hoc
            for root_trace_id in self.root_traces() {
                self.gen_trace_stalks_within_trace(sample_id, root_trace_id)?
            }
        }
        Ok(())
    }

    fn gen_trace_stalks_within_trace(
        &mut self,
        sample_id: SampleId,
        trace_id: TraceId,
    ) -> __VMResult<()> {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let _trace_stalk_key = TraceStalkKey::from_trace_data(sample_id, trace_raw_data);
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        self.gen_trace_stalk(trace_id)?;
        for associated_trace_id in associated_trace_ids {
            self.gen_trace_stalks_within_trace(sample_id, associated_trace_id)?
        }
        if expanded {
            for subtrace_id in self.subtraces(trace_id) {
                self.gen_trace_stalks_within_trace(sample_id, subtrace_id)?
            }
        }
        Ok(())
    }

    fn trace_stalk_from_expr(&self, expr: ValExpr, sample_id: SampleId) -> TraceStalk {
        todo!()
        // let arrived = match self
        //     .runtime
        //     .eval_opt_domain_indicator_cached(expr.opt_arrival_indicator.as_ref(), sample_id)
        // {
        //     Ok(arrived) => arrived,
        //     Err(_) => false,
        // };
        // if arrived {
        //     self.trace_stalk_from_result(
        //         self.runtime().eval_feature_expr(expr, sample_id),
        //         expr.expr.intrinsic_ty(),
        //     )
        // } else {
        //     TraceStalk::unarrived()
        // }
    }

    fn trace_stalk_from_result(
        &self,
        result: __VMResult<RegularValue>,
        ty: EtherealTerm,
    ) -> TraceStalk {
        TraceStalk {
            extra_tokens: vec![
                TraceTokenData {
                    kind: TraceTokenKind::Fade,
                    value: " = ".to_string(),
                    opt_associated_trace_id: None,
                },
                self.trace_token_from_result(result, ty),
            ],
            kind: TraceStalkKind::Value,
        }
    }

    pub(crate) fn trace_token_from_result(
        &self,
        result: __VMResult<RegularValue>,
        ty: EtherealTerm,
    ) -> TraceTokenData {
        match result {
            Ok(value) => self.trace_token_from_value(value, ty),
            Err(e) => TraceTokenData {
                kind: TraceTokenKind::Error,
                value: e.message().to_string(),
                opt_associated_trace_id: None,
            },
        }
    }

    pub(crate) fn trace_token_from_value(
        &self,
        value: RegularValue,
        ty: EtherealTerm,
    ) -> TraceTokenData {
        todo!()
        // TraceTokenData {
        //     kind: TraceTokenKind::Fade,
        //     value: self.runtime().print_short(&value, ty),
        //     opt_associated_trace_id: None,
        // }
    }
}
