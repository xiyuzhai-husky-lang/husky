use semantics_entity::EntityDefnVariant;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &FeatureExpr,
    ) -> Vec<TraceId> {
        match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(_)
            | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureLazyExprVariant::Variable { .. } => vec![],
            FeatureLazyExprVariant::RoutineCall {
                ref opt_instruction_sheet,
                ref routine_defn,
                ref opds,
                ..
            } => self.routine_call_subtraces(
                parent,
                opt_instruction_sheet.as_ref().unwrap(),
                routine_defn,
                opds,
                |this, argument, ident| {
                    (
                        this.new_trace(
                            Some(parent.id()),
                            4,
                            TraceVariant::FeatureCallArgument {
                                argument: argument.clone(),
                                ident,
                            },
                        ),
                        this.eval_time_singleton
                            .eval_feature_expr(argument, this.attention.opt_sample_id().unwrap()),
                    )
                },
            ),
            FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureLazyExprVariant::StructOriginalFieldAccess { .. } => panic!(),
            FeatureLazyExprVariant::EnumKindLiteral { .. } => panic!(),
            FeatureLazyExprVariant::EvalInput => panic!(),
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => panic!(),
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureLazyExprVariant::ModelCall {
                ref opds,
                has_this,
                ref model_defn,
                ref internal,
            } => todo!(),
        }
    }
}
