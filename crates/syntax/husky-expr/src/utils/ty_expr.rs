use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeExpr {
    expr: ExprIdx,
}

impl TypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for TypeExpr {
    type Error = ExprError;

    fn try_parse_optional_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::SelfType) {
            Ok(Some(TypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
