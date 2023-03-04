use super::*;
use husky_entity_tree::RegionPath;
use husky_expr::{CurrentSymbolIdx, ExprIdx, ExprMap, ExprRegion};

#[derive(Debug, PartialEq, Eq)]
pub struct RawSignatureRawTermRegion {
    path: RegionPath,
    term_symbol_region: RawTermSymbolRegion,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
}

impl RawSignatureRawTermRegion {
    pub fn new(
        path: RegionPath,
        term_symbol_region: RawTermSymbolRegion,
        expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
        }
    }

    pub fn term_symbol_region(&self) -> &RawTermSymbolRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.term_symbol_region
            .current_symbol_term(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: ExprIdx) -> SignatureRawTermResultBorrowed<RawTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }
}

#[salsa::tracked(jar = RawSignatureJar, return_ref)]
pub(crate) fn raw_signature_term_region(
    db: &dyn RawSignatureDb,
    expr_region: ExprRegion,
) -> RawSignatureRawTermRegion {
    let expr_region_data = expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| raw_signature_term_region(db, r).term_symbol_region());
    RawSignatureRawTermEngine::new(db, expr_region, parent_term_symbol_region).finish()
}
