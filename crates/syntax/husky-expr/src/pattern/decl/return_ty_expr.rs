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

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for ReturnTypeExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr(None) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::ReturnType, expr));
            Ok(Some(ReturnTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
