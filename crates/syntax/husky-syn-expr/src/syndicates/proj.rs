use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ProjSyndicate {
    syn_expr_idx: SynExprIdx,
}

impl ProjSyndicate {
    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

impl<'a> TryParseOptionFromStream<StandaloneSynExprParser<'a>> for ProjSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::Projection) {
            Ok(Some(ProjSyndicate { syn_expr_idx: expr }))
        } else {
            Ok(None)
        }
    }
}
