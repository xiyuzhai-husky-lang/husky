use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitObelisk {
    expr: SynExprIdx,
}

impl TraitObelisk {
    pub fn expr(&self) -> ArenaIdx<SynExpr> {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for TraitObelisk {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::Trait) {
            Ok(Some(TraitObelisk { expr }))
        } else {
            Ok(None)
        }
    }
}
