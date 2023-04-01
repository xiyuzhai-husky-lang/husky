use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VarTypeExpr {
    expr: ExprIdx,
}

impl VarTypeExpr {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for VarTypeExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr(Some(ExprEnvironment::TypeBeforeEq)) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::VarType, expr));
            Ok(Some(VarTypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
