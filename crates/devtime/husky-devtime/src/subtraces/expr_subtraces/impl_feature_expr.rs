use super::*;

impl Devtime {
    pub(crate) fn feature_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: ValExpr,
    ) -> Option<Vec<TraceId>> {
        todo!()
        // match expr.variant {
        //     FeatureLazyExprVariant::Literal(_)
        //     | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
        //     | FeatureLazyExprVariant::Variable { .. }
        //     | FeatureLazyExprVariant::ShortCircuitBinaryOpr { .. } => None,
        //     FeatureLazyExprVariant::RoutineCall {
        //         ref opt_instruction_sheet,
        //         ref routine_defn,
        //         ref opds,
        //         ..
        //     } => Some(self.routine_call_subtraces(
        //         parent,
        //         opt_instruction_sheet.as_ref().unwrap(),
        //         routine_defn,
        //         opds,
        //         |this, argument, ident| {
        //             (
        //                 this.new_trace(
        //                     Some(parent.id()),
        //                     4,
        //                     TraceVariant::ValCallArgument {
        //                         argument: argument.clone(),
        //                         name: ident,
        //                     },
        //                 ),
        //                 Some(this.runtime.eval_feature_expr(
        //                     argument,
        //                     this.state.presentation().opt_sample_id().unwrap(),
        //                 )),
        //             )
        //         },
        //     )),
        //     FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
        //         self.feature_repr_subtraces(parent, repr)
        //     }
        //     FeatureLazyExprVariant::NewRecord { .. } => todo!(),
        //     FeatureLazyExprVariant::RecordOriginalField { .. } => todo!(),
        //     FeatureLazyExprVariant::ThisValue { .. } => todo!(),
        //     FeatureLazyExprVariant::RecordDerivedField { .. } => todo!(),
        //     FeatureLazyExprVariant::StructOriginalField { .. } => panic!(),
        //     FeatureLazyExprVariant::EvalInput => panic!(),
        //     FeatureLazyExprVariant::Index { .. } => panic!(),
        //     FeatureLazyExprVariant::StructDerivedLazyField { ref repr, .. } => {
        //         self.feature_repr_subtraces(parent, repr)
        //     }
        //     FeatureLazyExprVariant::ModelCall { .. } => todo!(),
        //     FeatureLazyExprVariant::NewVecFromList { .. } => todo!(),
        //     FeatureLazyExprVariant::CustomBinaryOpr { .. } => todo!(),
        //     FeatureLazyExprVariant::BePattern { .. } => todo!(),
        //     FeatureLazyExprVariant::PrefixOpr { .. } => todo!(),
        // }
    }
}
