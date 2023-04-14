mod error;
mod symbol;

pub use self::error::*;
pub use self::symbol::*;

pub(crate) use engine::*;

use crate::*;

use husky_entity_tree::{DeclRegionPath, ImplBlockId, RegionPath};
use husky_expr::{
    AllowSelfType, CurrentSymbolIdx, ExprIdx, ExprMap, ExprRegion, InheritedSymbolIdx,
    ParentSymbolIdx, PatternExprIdx, PatternExprMap, PatternSymbolMap, SymbolRegion,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
#[derive(Debug, PartialEq, Eq)]
pub struct SignatureRegion {
    path: RegionPath,
    term_symbol_region: SymbolRawTermRegion,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    pattern_expr_ty_infos: PatternExprMap<PatternExprRawTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
}

impl SignatureRegion {
    pub(crate) fn new(
        path: RegionPath,
        term_symbol_region: SymbolRawTermRegion,
        expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
        pattern_expr_ty_infos: PatternExprMap<PatternExprRawTypeInfo>,
        pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
        }
    }

    pub fn term_symbol_region(&self) -> &SymbolRawTermRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.term_symbol_region
            .current_symbol_signature(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: ExprIdx) -> SignatureRawTermResultBorrowed<RawTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }
}
