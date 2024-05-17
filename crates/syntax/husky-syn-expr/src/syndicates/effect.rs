use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EffectSyndicate {
    expr: SynExprIdx,
}

impl EffectSyndicate {
    pub fn expr(self) -> SynExprIdx {
        self.expr
    }
}

impl<'a> TryParseOptionFromStream<SynDeclExprParser<'a>> for EffectSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::Effect) {
            Ok(Some(EffectSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
