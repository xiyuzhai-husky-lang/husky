use super::*;

/// used for memo, var, const, constexpr
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FormTypeExpr {
    expr: ExprIdx,
}

impl FormTypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> ParseFromStreamWithError<ExprParseContext<'a, 'b>> for FormTypeExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr(Some(ExprEnvironment::TypeBeforeEq)) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::VarType, expr));
            Ok(Some(FormTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
