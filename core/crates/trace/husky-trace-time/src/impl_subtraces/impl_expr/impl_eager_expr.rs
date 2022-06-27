use super::*;

impl HuskyTraceTime {
    pub(crate) fn eager_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &EagerExpr,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceId> {
        match expr.variant {
            EagerExprVariant::Variable { .. } => todo!(),
            EagerExprVariant::ThisValue { .. } => todo!(),
            EagerExprVariant::ThisField {
                field_ident,
                field_idx,
                this_ty,
                this_binding,
                field_binding,
            } => todo!(),
            EagerExprVariant::EntityRoute { route } => todo!(),
            EagerExprVariant::PrimitiveLiteral(_) => todo!(),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => todo!(),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
        }
    }
}
