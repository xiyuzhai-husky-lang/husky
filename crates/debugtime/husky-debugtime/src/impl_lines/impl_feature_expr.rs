use super::*;
use crate::*;
use husky_entity_route::RangedEntityRoute;
use husky_lazy_semantics::{LazyExpr, LazyExprVariant, LazyOpnKind};
use husky_pattern_semantics::{PurePattern, PurePatternVariant};
use husky_text::RangedCustomIdentifier;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn gen_feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) {
        let opt_assoc = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone())))
        } else {
            None
        };
        match expr.variant {
            FeatureLazyExprVariant::Literal(_) => match expr.expr.variant {
                LazyExprVariant::PrimitiveLiteral(value) => {
                    self.gen_literal_token(value, Some(expr.expr.range.start))
                }
                LazyExprVariant::EnumLiteral { .. } => {
                    let text = self.runtime().text(expr.expr.file).unwrap();
                    self.gen_route_token(
                        text.ranged(expr.expr.range),
                        opt_assoc,
                        Some(expr.expr.range.start),
                    )
                }
                _ => panic!(),
            },
            FeatureLazyExprVariant::PrimitiveBinaryOpr { opr, ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.gen_special_token(opr.spaced_husky_code(), opt_assoc, None);
                self.gen_feature_expr_tokens(&opds[1], config.subexpr())
            }
            FeatureLazyExprVariant::ShortCircuitBinaryOpr { opr, ref opds } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.gen_special_token(opr.spaced_husky_code(), opt_assoc, None);
                self.gen_feature_expr_tokens(&opds[1], config.subexpr())
            }
            FeatureLazyExprVariant::CustomBinaryOpr { opr, ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.gen_special_token(opr.spaced_husky_code(), opt_assoc, None);
                self.gen_feature_expr_tokens(&opds[1], config.subexpr())
            }
            FeatureLazyExprVariant::Variable { varname, .. } => {
                self.gen_ident_token(varname.0, opt_assoc, Some(expr.expr.range.start))
            }
            FeatureLazyExprVariant::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => self
                        .feature_entity_call_tokens(
                            expr.expr.file,
                            ranged_route,
                            feature_opds,
                            opt_assoc,
                            config,
                            &expr.expr,
                        ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::MethodCall { method_ident, .. } => {
                        self.gen_feature_expr_tokens(&feature_opds[0], config.subexpr());
                        self.gen_special_token(".", None, Some(method_ident.range.start));
                        self.gen_ident_token(method_ident.ident.0, None, None);
                        self.gen_special_token("(", None, None);
                        for i in 1..opds.len() {
                            if i > 1 {
                                self.gen_special_token(", ", None, None)
                            }
                            self.gen_feature_expr_tokens(&feature_opds[i], config.subexpr());
                        }
                        self.gen_special_token(")", None, None);
                    }
                    _ => panic!(),
                },
                _ => panic!(""),
            },
            FeatureLazyExprVariant::ModelCall { ref opds, .. } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, .. } => match opn_kind {
                    LazyOpnKind::FunctionModelCall(route) => self.feature_entity_call_tokens(
                        expr.expr.file,
                        route,
                        opds,
                        opt_assoc,
                        config,
                        &expr.expr,
                    ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::Field { .. } => todo!(),
                    LazyOpnKind::MethodCall { .. } => todo!(),
                    LazyOpnKind::Index { .. } => todo!(),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureLazyExprVariant::EntityFeature { .. } => {
                let text = self.runtime.text(expr.expr.file).unwrap();
                self.gen_route_token(
                    text.ranged(expr.expr.range),
                    opt_assoc,
                    Some(expr.expr.range.start),
                )
            }
            FeatureLazyExprVariant::NewRecord { .. } => todo!(),
            FeatureLazyExprVariant::ThisValue { .. } => todo!(),
            FeatureLazyExprVariant::EvalInput => self.gen_keyword_token("input", None, None),
            FeatureLazyExprVariant::Index { ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.gen_special_token("[", opt_assoc.clone(), None);
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    self.gen_feature_expr_tokens(index_opd, config.subexpr());
                }
                self.gen_special_token("]", opt_assoc, None)
            }
            FeatureLazyExprVariant::RecordDerivedField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident, opt_assoc),
            FeatureLazyExprVariant::StructOriginalField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident, opt_assoc),
            FeatureLazyExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident, opt_assoc),
            FeatureLazyExprVariant::StructDerivedLazyField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_lazy_field_tokens(this, field_ident, opt_assoc, config),
            FeatureLazyExprVariant::NewVecFromList { ref elements, .. } => {
                self.gen_new_vec_from_list_tokens(elements, opt_assoc, config)
            }
            FeatureLazyExprVariant::BePattern { ref this, ref patt } => {
                self.gen_be_pattern(this, patt, config)
            }
        }
    }

    fn gen_new_vec_from_list_tokens(
        &mut self,
        elements: &[Arc<FeatureLazyExpr>],
        opt_assoc_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        self.gen_special_token("[", opt_assoc_id, None);
        for (i, element) in elements.iter().enumerate() {
            if i > 0 {
                self.gen_special_token(", ", None, None)
            }
            self.gen_feature_expr_tokens(element, config.subexpr())
        }
        self.gen_special_token("]", opt_assoc_id, None);
    }

    fn gen_feature_eager_field_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
        opt_associated_trace_id: Option<TraceId>,
    ) {
        match this {
            FeatureRepr::LazyExpr(this) => {
                self.gen_feature_expr_tokens(this, config.subexpr());
                self.gen_special_token(".", None, Some(field_ident.range.start));
                self.gen_ident_token(field_ident.ident.as_str(), opt_associated_trace_id, None)
            }
            _ => self.gen_ident_token(
                field_ident.ident.as_str(),
                None,
                Some(field_ident.range.start),
            ),
        }
    }

    fn gen_feature_lazy_field_tokens(
        &mut self,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        match this {
            FeatureRepr::LazyExpr(this) => {
                self.gen_feature_expr_tokens(this, config.subexpr());
                self.gen_special_token(".", None, Some(field_ident.range.start));
                self.gen_ident_token(field_ident.ident.as_str(), opt_associated_trace_id, None);
            }
            _ => self.gen_ident_token(
                field_ident.ident.as_str(),
                None,
                Some(field_ident.range.start),
            ),
        }
    }

    fn feature_entity_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureLazyExpr>],
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
        expr: &LazyExpr,
    ) {
        let text = self.runtime().text(file).unwrap();
        self.gen_route_token(
            text.ranged(ranged_scope.range),
            opt_associated_trace_id,
            Some(ranged_scope.range.start),
        );
        self.gen_special_token("(", None, Some(expr.range.start));
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                self.gen_special_token(", ", None, None);
            }
            self.gen_feature_expr_tokens(input, config.subexpr());
        }
        self.gen_special_token(")", None, Some(expr.range.end.to_left(1)))
    }

    fn gen_be_pattern(
        &mut self,
        this: &Arc<FeatureLazyExpr>,
        patt: &PurePattern,
        config: ExprTokenConfig,
    ) {
        self.gen_feature_expr_tokens(this, config.subexpr());
        self.gen_special_token(" be ", None, None);
        self.gen_pattern(patt)
    }

    fn gen_pattern(&mut self, patt: &PurePattern) {
        match patt.variant {
            PurePatternVariant::PrimitiveLiteral(_) => todo!(),
            PurePatternVariant::OneOf { .. } => todo!(),
            PurePatternVariant::EnumLiteral(_) => todo!(),
            PurePatternVariant::Some => self.gen_keyword_token("some", None, None),
            PurePatternVariant::None => self.gen_keyword_token("none", None, None),
        }
    }
}
