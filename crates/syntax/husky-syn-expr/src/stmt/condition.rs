use super::*;


impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_condition(&mut self, block_end: RegionalTokenIdxRangeEnd) -> SynExprIdx {
        self.parse_expr_expected2(
            Some(ExprEnvironment::Condition(block_end)),
            SynExprRootKind::Condition,
            OriginalSynExprError::ExpectedCondition,
        )
    }
}
//
