use husky_entity_semantics::EntityDefnVariant;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &FeatureLazyExpr,
    ) -> Option<Vec<TraceId>> {
        match expr.variant {
            FeatureLazyExprVariant::Literal(_)
            | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureLazyExprVariant::Variable { .. } => None,
            FeatureLazyExprVariant::RoutineCall {
                ref opt_instruction_sheet,
                ref routine_defn,
                ref opds,
                ..
            } => Some(self.routine_call_subtraces(
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
                                name: ident,
                            },
                        ),
                        Some(this.runtime.eval_feature_expr(
                            argument,
                            this.restriction.opt_sample_id().unwrap(),
                        )),
                    )
                },
            )),
            FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::RecordDerivedField { .. } => todo!(),
            FeatureLazyExprVariant::StructOriginalField { .. } => panic!(),
            FeatureLazyExprVariant::EvalInput => panic!(),
            FeatureLazyExprVariant::Index { ref opds, .. } => panic!(),
            FeatureLazyExprVariant::StructDerivedLazyField { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureLazyExprVariant::ModelCall {
                ref opds,
                has_this,
                ref model_defn,
                ref internal,
                ..
            } => todo!(),
            FeatureLazyExprVariant::NewVecFromList { .. } => todo!(),
            FeatureLazyExprVariant::CustomBinaryOpr { .. } => todo!(),
            FeatureLazyExprVariant::BePattern {
                ref this,
                patt: ref pure_pattern,
            } => todo!(),
        }
    }
}
