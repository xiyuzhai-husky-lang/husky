use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitExpr {
    expr: ExprIdx,
}

impl<'a, 'b> ParseFrom<ExprParseContext<'a, 'b>> for TraitExpr {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        if let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) {
            ctx.add_ty_constraint(TypeConstraint::Trait { trai: expr });
            Ok(Some(TraitExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
