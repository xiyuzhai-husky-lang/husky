use entity_route::RangedEntityRoute;

use super::expr::ExprTokenConfig;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub(crate) fn feature_expr_lines(
        &self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, text, config),
        }]
    }

    pub(crate) fn feature_expr_tokens(
        &self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let associated_trace = if config.associated {
            Some(self.new_trace(None, 0, TraceKind::FeatureExpr(expr.clone()), text))
        } else {
            None
        };
        return match expr.kind {
            FeatureExprKind::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, text, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.feature_expr_tokens(ropd, text, config.subexpr()));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
            FeatureExprKind::FuncCall {
                func_ranged_scope: ranged_scope,
                ref inputs,
                ..
            } => self.routine_call_tokens(ranged_scope, inputs, associated_trace, text, &config),
            FeatureExprKind::ProcCall {
                proc_ranged_scope: ranged_scope,
                ref inputs,
                ..
            } => self.routine_call_tokens(ranged_scope, inputs, associated_trace, text, &config),
            FeatureExprKind::StructMembVarAccess { .. } => todo!(),
            FeatureExprKind::EnumLiteral { .. } => todo!(),
            FeatureExprKind::MethodCall {
                field_ident,
                ref opds,
                ref instruction_sheet,
                ref stmts,
                ..
            } => todo!(),
            FeatureExprKind::MembProcCall {
                field_ident,
                ref opds,
                ref instruction_sheet,
                ref stmts,
                ..
            } => todo!(),
            FeatureExprKind::MembPattCall {
                field_ident,
                ref opds,
                ref instruction_sheet,
                ref stmts,
            } => todo!(),
            FeatureExprKind::FeatureBlock { .. } => todo!(),
            FeatureExprKind::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprKind::RecordMembAccess {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => vec![keyword!("input")],
        };
    }

    fn routine_call_tokens(
        &self,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureExpr>],
        associated_trace: Option<Arc<Trace<'eval>>>,
        text: &Text,
        config: &ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let mut tokens = vec![
            scope!(text.ranged(ranged_scope.range), associated_trace),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.feature_expr_tokens(input, text, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }
}
