use semantics_entity::EntityDefnVariant;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &FeatureLazyExpr,
    ) -> Vec<TraceId> {
        match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(_)
            | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
            | FeatureLazyExprVariant::Variable { .. } => vec![],
            FeatureLazyExprVariant::RoutineCall {
                ref opt_instruction_sheet,
                ref routine_defn,
                ref opds,
                has_this,
                ..
            } => {
                if let Some(sample_id) = self.attention.opt_sample_id() {
                    let instruction_sheet: &InstructionSheet =
                        opt_instruction_sheet.as_ref().unwrap();
                    let mut subtraces = vec![];
                    let mut func_input_values = vec![];
                    subtraces.push(self.new_call_head(parent, routine_defn.clone()));
                    let parameters: &[Parameter] = match routine_defn.variant {
                        EntityDefnVariant::Func { ref parameters, .. } => parameters,
                        EntityDefnVariant::Proc { ref parameters, .. } => parameters,
                        _ => panic!(),
                    };
                    for (i, func_input) in opds.iter().enumerate() {
                        subtraces.push(self.new_trace(
                            Some(parent.id()),
                            4,
                            TraceVariant::FeatureCallArgument {
                                argument: func_input.clone(),
                                ident: parameters[i].ranged_ident.ident,
                            },
                        ));
                        match self
                            .eval_time_singleton
                            .eval_feature_lazy_expr(func_input, sample_id)
                        {
                            Ok(value) => func_input_values.push(value.into_stack().unwrap()),
                            Err(_) => return subtraces,
                        }
                    }
                    let history = exec_debug(
                        self.eval_time().upcast(),
                        instruction_sheet,
                        func_input_values.into_iter(),
                        self.eval_time().verbose(),
                    );
                    match routine_defn.variant {
                        EntityDefnVariant::Func { ref stmts, .. } => {
                            subtraces.extend(self.func_stmts_traces(
                                parent.id(),
                                4,
                                stmts,
                                &history,
                            ));
                        }
                        EntityDefnVariant::Proc { ref stmts, .. } => {
                            subtraces.extend(self.proc_stmts_traces(
                                parent.id(),
                                4,
                                stmts,
                                &history,
                            ));
                        }
                        _ => panic!(),
                    }
                    subtraces
                } else {
                    vec![]
                }
            }
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
