use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitExpr {
    expr: SynExprIdx,
}

impl TraitExpr {
    pub fn expr(&self) -> ArenaIdx<SynExpr> {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for TraitExpr {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::Trait) {
            Ok(Some(TraitExpr { expr }))
        } else {
            Ok(None)
        }
    }
}
