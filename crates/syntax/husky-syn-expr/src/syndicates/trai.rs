use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitSyndicate {
    expr: SynExprIdx,
}

impl TraitSyndicate {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for TraitSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::Trait) {
            Ok(Some(TraitSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
