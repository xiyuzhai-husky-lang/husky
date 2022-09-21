use husky_entity_route::RangedEntityRoute;

use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn gen_eager_expr_tokens(
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
                self.gen_ident_token(varname.0, associated_trace_id, Some(expr.range.start))
            }
            EagerExprVariant::PrimitiveLiteral(value) => {
                self.gen_literal_token(value, Some(expr.range.start))
            }
            EagerExprVariant::Bracketed(ref expr) => {
                self.gen_special_token("(", None, Some(expr.range.start));
                self.gen_eager_expr_tokens(expr, history, config.subexpr());
                self.gen_special_token(")", None, Some(expr.range.closed_end()));
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr } => {
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.gen_special_token(opr.spaced_code(), associated_trace_id, None);
                    self.gen_eager_expr_tokens(&opds[1], history, config.subexpr());
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    self.gen_special_token(opr.code(), associated_trace_id, Some(expr.range.start));
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.gen_special_token(
                        &opr.code(),
                        associated_trace_id,
                        Some(expr.range.closed_end()),
                    );
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
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.gen_special_token(".", None, Some(field_ident.range.start.to_left(1)));
                    self.gen_ident_token(field_ident.ident.0, associated_trace_id, None);
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.gen_special_token(".", None, Some(method_ident.range.start.to_left(1)));
                    self.gen_ident_token(method_ident.ident.0, associated_trace_id, None);
                    self.gen_special_token("(", None, None);
                    for i in 1..opds.len() {
                        if i > 1 {
                            self.gen_special_token(", ", None, None)
                        }
                        self.gen_eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.gen_special_token(")", None, Some(expr.range.closed_end()));
                }
                EagerOpnVariant::Index { .. } => {
                    self.gen_eager_expr_tokens(&opds[0], history, config.subexpr());
                    self.gen_special_token("[", associated_trace_id.clone(), None);
                    for i in 1..opds.len() {
                        if i > 1 {
                            self.gen_special_token(", ", None, None)
                        }
                        self.gen_eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.gen_special_token("]", associated_trace_id, Some(expr.range.closed_end()));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    let text = self.runtime.text(expr.file).unwrap();
                    self.gen_route_token(
                        text.ranged(ranged_ty.range),
                        None,
                        Some(expr.range.start),
                    );
                    self.gen_special_token("(", None, None);
                    for i in 0..opds.len() {
                        if i > 0 {
                            self.gen_special_token(", ", None, None)
                        }
                        self.gen_eager_expr_tokens(&opds[i], history, config.subexpr());
                    }
                    self.gen_special_token(")", None, Some(expr.range.closed_end()));
                }
                EagerOpnVariant::NewVecFromList => todo!(),
                EagerOpnVariant::ValueCall => todo!(),
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { .. } => {
                self.gen_ident_token("this", None, Some(expr.range.start))
            }
            EagerExprVariant::ThisField { field_ident, .. } => {
                self.gen_ident_token(field_ident.ident.0, None, Some(field_ident.range.start))
            }
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
            EagerExprVariant::EntityFeature { .. } => {
                let text = self.runtime.text(expr.file).unwrap();
                self.gen_route_token(text.ranged(expr.range), None, Some(expr.range.closed_end()))
            }
            EagerExprVariant::EntityThickFp { .. } => todo!(),
        };
        if config.appended {
            self.gen_fade_assign_token();
            if let Some(result) = history.register_result(expr) {
                self.gen_result_token(result, expr.intrinsic_ty(), None)
            } else {
                self.gen_fade_token("???", None)
            }
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
        let text = self.runtime().text(file).unwrap();
        self.gen_route_token(
            text.ranged(ranged_scope.range),
            opt_associated_trace_id,
            Some(ranged_scope.range.start),
        );
        self.gen_special_token("(", None, None);
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                self.gen_special_token(", ", None, None);
            }
            self.gen_eager_expr_tokens(input, history, config.subexpr());
        }
        self.gen_special_token(")", None, Some(ranged_scope.range.closed_end()));
    }
}
