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
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr(None) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::SelfType, expr));
            Ok(Some(TypeExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
