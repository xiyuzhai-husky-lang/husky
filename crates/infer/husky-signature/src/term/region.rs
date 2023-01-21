use super::*;
use husky_expr::ExprRegion;

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermRegion {
    term_symbol_region: TermSymbolRegion,
}

impl SignatureTermRegion {
    pub fn new(term_symbol_region: TermSymbolRegion) -> Self {
        Self { term_symbol_region }
    }

    pub fn term_symbol_region(&self) -> &TermSymbolRegion {
        &self.term_symbol_region
    }
}

#[salsa::tracked(jar = SignatureJar, return_ref)]
pub(crate) fn signature_term_region(
    db: &dyn SignatureDb,
    expr_region: ExprRegion,
) -> SignatureTermRegion {
    let parent_expr_region = expr_region.parent(db);
    let parent_term_symbol_region =
        parent_expr_region.map(|r| signature_term_region(db, r).term_symbol_region());
    SignatureTermEngine::new(db, expr_region, parent_term_symbol_region).finish()
}
