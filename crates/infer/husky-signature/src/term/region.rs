use super::*;
use husky_expr::{CurrentSymbolIdx, ExprIdx, ExprMap, ExprPath, ExprRegion};

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermRegion {
    expr_path: ExprPath,
    term_symbol_region: TermSymbolRegion,
    expr_terms: ExprMap<SignatureTermResult<Term>>,
}

impl SignatureTermRegion {
    pub fn new(
        expr_path: ExprPath,
        term_symbol_region: TermSymbolRegion,
        expr_terms: ExprMap<SignatureTermResult<Term>>,
    ) -> Self {
        Self {
            expr_path,
            term_symbol_region,
            expr_terms,
        }
    }

    pub fn term_symbol_region(&self) -> &TermSymbolRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_term(&self, current_symbol_idx: CurrentSymbolIdx) -> TermSymbol {
        self.term_symbol_region
            .current_symbol_term(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: ExprIdx) -> SignatureTermResultBorrowed<Term> {
        self.expr_terms[expr].as_ref().map(|t| *t)
    }

    pub fn expr_path(&self) -> ExprPath {
        self.expr_path
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
