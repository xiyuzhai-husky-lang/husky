use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AffectSyndicate {
    expr: SynExprIdx,
}

impl AffectSyndicate {
    pub fn expr(self) -> SynExprIdx {
        self.expr
    }
}

impl<'a> TryParseOptionFromStream<StandaloneSynExprParser<'a>> for AffectSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::Effect) {
            Ok(Some(AffectSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
