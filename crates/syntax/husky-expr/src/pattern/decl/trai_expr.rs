use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitExpr {
    expr: ExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for TraitExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, OriginalExprError> {
        if let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) {
            ctx.add_expr_root(ExprRoot::new(ExprRootKind::Trait, expr));
            Ok(Some(TraitExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
