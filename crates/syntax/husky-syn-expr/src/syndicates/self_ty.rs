use super::*;
use husky_entity_tree::region_path::SynNodeRegionPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SelfTypeSyndicate {
    expr: SynExprIdx,
}

impl SelfTypeSyndicate {
    pub fn expr(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for SelfTypeSyndicate
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::SelfType) {
            Ok(Some(SelfTypeSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
