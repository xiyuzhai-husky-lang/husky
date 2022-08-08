use husky_entity_semantics::EntityDefnVariant;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &FeatureExpr,
    ) -> Option<Vec<TraceId>> {
        match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(_)
            | FeatureExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureExprVariant::Variable { .. } => None,
            FeatureExprVariant::RoutineCall {
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
                        Some(this.runtime_singleton.eval_feature_expr(
                            argument,
                            this.restriction.opt_sample_id().unwrap(),
                        )),
                    )
                },
            )),
            FeatureExprVariant::EntityFeature { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureExprVariant::ThisValue { ref repr } => todo!(),
            FeatureExprVariant::RecordDerivedField { .. } => todo!(),
            FeatureExprVariant::StructOriginalField { .. } => panic!(),
            FeatureExprVariant::EnumKindLiteral { .. } => panic!(),
            FeatureExprVariant::EvalInput => panic!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => panic!(),
            FeatureExprVariant::StructDerivedLazyField { ref repr, .. } => {
                self.feature_repr_subtraces(parent, repr)
            }
            FeatureExprVariant::ModelCall {
                ref opds,
                has_this,
                ref model_defn,
                ref internal,
                ..
            } => todo!(),
            FeatureExprVariant::NewVecFromList { .. } => todo!(),
            FeatureExprVariant::CustomBinaryOpr { .. } => todo!(),
        }
    }
}
