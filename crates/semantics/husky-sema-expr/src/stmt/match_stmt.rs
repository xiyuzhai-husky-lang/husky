use husky_regional_token::{HeavyArrowRegionalToken, VerticalRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaCaseBranch {
    pub vertical_token: VerticalRegionalToken,
    pub case_pattern: CasePatternSynObelisk,
    pub heavy_arrow_token: HeavyArrowRegionalToken,
    pub stmts: SemaStmtIdxRange,
}
