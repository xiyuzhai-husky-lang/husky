mod error;
mod liason;
mod term;

pub use error::*;
pub use term::*;

pub(crate) use engine::*;

use crate::*;

use husky_entity_tree::{DeclRegionPath, ImplBlockId, RegionPath};
use husky_expr::{
    AllowSelfType, CurrentSymbolIdx, ExprIdx, ExprMap, ExprRegion, InheritedSymbolIdx,
    ParentSymbolIdx, PatternExprMap, PatternSymbolMap, SymbolRegion,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
#[derive(Debug, PartialEq, Eq)]
pub struct SignatureRegion {
    path: RegionPath,
    term_symbol_region: RawTermSymbolRegion,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    liasons: PatternExprMap<Liason>,
    pattern_expr_ty_infos: PatternExprMap<PatternExprRawTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
}

impl SignatureRegion {
    pub(crate) fn new(
        path: RegionPath,
        term_symbol_region: RawTermSymbolRegion,
        expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
        liasons: PatternExprMap<Liason>,
        pattern_expr_ty_infos: PatternExprMap<PatternExprRawTypeInfo>,
        pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
            liasons,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
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
