use super::*;
use husky_entity_tree::region_path::SynNodeRegionPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeColonSyndicate {
    syn_expr_idx: SynExprIdx,
}

impl ReturnTypeBeforeColonSyndicate {
    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

impl<'a, 'b> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for ReturnTypeBeforeColonSyndicate
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, SynExprRootKind::ReturnType) {
            Ok(Some(ReturnTypeBeforeColonSyndicate { syn_expr_idx: expr }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeBeforeEqSyndicate {
    expr: SynExprIdx,
}

impl ReturnTypeBeforeEqSyndicate {
    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<StandaloneSynExprParser<'a, SynNodeRegionPath>>
    for ReturnTypeBeforeEqSyndicate
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut StandaloneSynExprParser<'a, SynNodeRegionPath>,
    ) -> SynExprResult<Option<Self>> {
        if let Some(expr) =
            ctx.parse_expr_root(ExprEnvironment::TypeBeforeEq, SynExprRootKind::ReturnType)
        {
            Ok(Some(ReturnTypeBeforeEqSyndicate { expr }))
        } else {
            Ok(None)
        }
    }
}
