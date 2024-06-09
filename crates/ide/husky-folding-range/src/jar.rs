use super::folding_ranges;

#[salsa::jar]
pub struct FoldingRangeJar(folding_ranges);
