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

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for TypeExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::Type, expr));
            Ok(Some(TypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
