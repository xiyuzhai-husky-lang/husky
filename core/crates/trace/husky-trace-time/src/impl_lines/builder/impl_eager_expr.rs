use husky_entity_route::RangedEntityRoute;

use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn eager_expr_tokens(
        &mut self,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        config: ExprTokenConfig,
    ) {
        let associated_trace_id = if config.associated {
            Some(self.new_eager_expr_trace(expr.clone(), history.clone(), None, 0))
        } else {
            None
        };
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => {
                self.push(ident!(varname.0, associated_trace_id))
            }
            EagerExprVariant::PrimitiveLiteral(value) => self.push(literal!(value)),
            EagerExprVariant::Bracketed(ref expr) => {
                self.push(special!("("));
                self.eager_expr_tokens(expr, history, config.subexpr());
                self.push(special!(")"));
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.push(special!(opr.spaced_code(), associated_trace_id));
                    self.eager_expr_tokens(&opds[1], history, config.subexpr());
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    self.push(special!(opr.code(), associated_trace_id));
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.push(special!(opr.code(), associated_trace_id));
                }
                EagerOpnVariant::RoutineCall(ranged_scope) => self.eager_routine_call_tokens(
                    expr.file,
                    *ranged_scope,
                    opds,
                    associated_trace_id,
                    history,
                    &config,
                ),
                EagerOpnVariant::Field { field_ident, .. } => {
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.push(special!("."));
                    self.push(ident!(field_ident.ident.0, associated_trace_id));
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.push(special!("."));
                    self.push(ident!(method_ident.ident.0, associated_trace_id));
                    self.push(special!("("));
                    for i in 1..opds.len() {
                        if i > 1 {
                            self.push(special!(", "))
                        }
                        self.eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.push(special!(")"));
                }
                EagerOpnVariant::Index { element_binding } => {
                    self.eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.push(special!("[", associated_trace_id.clone()));
                    for i in 1..opds.len() {
                        if i > 1 {
                            self.push(special!(", "))
                        }
                        self.eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.push(special!("]", associated_trace_id));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    let text = self
                        .eval_time_singleton
                        .compile_time()
                        .text(expr.file)
                        .unwrap();
                    self.push(route!(text.ranged(ranged_ty.range)));
                    self.push(special!("("));
                    for i in 0..opds.len() {
                        if i > 0 {
                            self.push(special!(", "))
                        }
                        self.eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.push(special!(")"));
                }
                EagerOpnVariant::NewVecFromList => todo!(),
                EagerOpnVariant::ValueCall => todo!(),
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { .. } => self.push(ident!("this")),
            EagerExprVariant::ThisField { field_ident, .. } => {
                self.push(ident!(field_ident.ident.0))
            }
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
            EagerExprVariant::EntityFeature { .. } => todo!(),
            EagerExprVariant::EntityFp { route } => todo!(),
        };
        if config.appended {
            self.push(fade!(" = "));
            self.push(history.register_result(expr).into())
        }
    }

    fn eager_routine_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<EagerExpr>],
        opt_associated_trace_id: Option<TraceId>,
        history: &Arc<History<'static>>,
        config: &ExprTokenConfig,
    ) {
        let text = self.eval_time().compile_time().text(file).unwrap();
        self.extend([
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ]);
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                self.push(special!(", "));
            }
            self.eager_expr_tokens(input, history, config.subexpr());
        }
        self.push(special!(")"));
    }
}
