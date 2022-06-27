use entity_route::RangedEntityRoute;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn eager_expr_tokens(
        &mut self,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let associated_trace_id = if config.associated {
            Some(self.new_eager_expr_trace(expr.clone(), history.clone(), None, 0))
        } else {
            None
        };
        let mut tokens = vec![];
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => {
                tokens.push(ident!(varname.0, associated_trace_id))
            }
            EagerExprVariant::EntityRoute { route: scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => return vec![literal!(value)],
            EagerExprVariant::Bracketed(ref expr) => {
                tokens.push(special!("("));
                tokens.extend(self.eager_expr_tokens(expr, history, config.subexpr()));
                tokens.push(special!(")"));
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!(opr.spaced_code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(&opds[1], history, config.subexpr()));
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    tokens.push(special!(opr.code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!(opr.code(), associated_trace_id));
                }
                EagerOpnVariant::RoutineCall(ranged_scope) => {
                    tokens = self.eager_routine_call_tokens(
                        expr.file,
                        *ranged_scope,
                        opds,
                        associated_trace_id,
                        history,
                        &config,
                    )
                }
                EagerOpnVariant::FieldAccess { field_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("."));
                    tokens.push(ident!(field_ident.ident.0));
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("."));
                    tokens.push(ident!(method_ident.ident.0));
                    tokens.push(special!("("));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!(")"));
                }
                EagerOpnVariant::ElementAccess { element_binding } => {
                    tokens.extend(self.eager_expr_tokens(&opds[0], history, config.subexpr()));
                    tokens.push(special!("[", associated_trace_id.clone()));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!("]", associated_trace_id));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    let text = self
                        .eval_time_singleton
                        .compile_time()
                        .text(expr.file)
                        .unwrap();
                    tokens.push(route!(text.ranged(ranged_ty.range)));
                    tokens.push(special!("("));
                    for i in 0..opds.len() {
                        if i > 0 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(&opds[i], history, config.subexpr()));
                    }
                    tokens.push(special!(")"));
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { .. } => tokens.push(ident!("this")),
            EagerExprVariant::ThisField { field_ident, .. } => {
                tokens.push(ident!(field_ident.ident.0))
            }
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
        };
        if config.appended {
            tokens.push(fade!(" = "));
            tokens.push(history.value_result(expr).into())
        }
        tokens
    }

    fn eager_routine_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<EagerExpr>],
        opt_associated_trace_id: Option<TraceId>,
        history: &Arc<History<'static>>,
        config: &ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let text = self.eval_time().compile_time().text(file).unwrap();
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.eager_expr_tokens(input, history, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }
}
