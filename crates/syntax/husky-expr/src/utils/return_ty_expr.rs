use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeExpr {
    expr: ExprIdx,
}

impl ReturnTypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for ReturnTypeExpr {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::ReturnType) {
            Ok(Some(ReturnTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
