use super::*;
use husky_entity_tree::region_path::SynNodeRegionPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitSyndicate {
    syn_expr_idx: SynExprIdx,
}

impl TraitSyndicate {
    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

impl<'a> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for TraitSyndicate
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::PrimalTrait) {
            Ok(Some(TraitSyndicate { syn_expr_idx: expr }))
        } else {
            Ok(None)
        }
    }
}
