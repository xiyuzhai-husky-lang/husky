use super::*;
use helpers::range::SemExprRangeRegionData;
use husky_expr::stmt::ConditionConversion;
use husky_regional_token::{RegionalTokenIdx, RegionalTokenIdxRange};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemCondition {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        expr: SemExprIdx,
        src: SemExprIdx,
        contract: Contract,
        be_regional_token_idx: RegionalTokenIdx,
        target: BePatternSyndicate,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        expr: SemExprIdx,
        conversion: ConditionConversion,
    },
}

impl SemCondition {
    pub fn regional_token_idx_range(
        self,
        sem_expr_range_region_data: &SemExprRangeRegionData,
        syn_expr_range_region: &SynExprRangeRegion,
    ) -> RegionalTokenIdxRange {
        match self {
            SemCondition::Be { src, target, .. } => sem_expr_range_region_data[src]
                .join(syn_expr_range_region[target.syn_pattern_root().syn_pattern_idx()]),
            SemCondition::Other {
                expr: sem_expr_idx, ..
            } => sem_expr_range_region_data[sem_expr_idx],
        }
    }
}
