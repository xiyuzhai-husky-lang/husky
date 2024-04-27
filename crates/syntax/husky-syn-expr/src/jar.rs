#[salsa::jar]
pub struct SynExprJar(
    crate::region::SynExprRegion,
    crate::snippet::parse_expr_from_snippet,
    crate::range::syn_expr_range_region,
);
