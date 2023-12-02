use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitSyndicate {
    syn_expr_idx: SynExprIdx,
}

impl TraitSyndicate {
    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

impl<'a, 'b> TryParseOptionFromStream<SynDeclExprParser<'a>> for TraitSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::Trait) {
            Ok(Some(TraitSyndicate { syn_expr_idx: expr }))
        } else {
            Ok(None)
        }
    }
}
