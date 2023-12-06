// todo: move this to husky-declarative-term
mod error;
mod symbol;

pub use self::error::*;
pub use self::symbol::*;

pub(crate) use engine::*;

use crate::*;

use husky_entity_syn_tree::SynNodeRegionPath;
use husky_syn_expr::{
    AllowSelfType, CurrentSynSymbolIdx, InheritedSynSymbolIdx, ParentSynSymbolIdx, SynExprIdx,
    SynExprMap, SynExprRegion, SynPatternExprIdx, SynPatternExprMap, SynPatternSymbolMap,
    SynSymbolRegionData,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
///
/// should contains term information enough for generating declarations
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermRegion {
    path: SynNodeRegionPath,
    term_symbol_region: SymbolDeclarativeTermRegion,
    expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolDeclarativeTypeInfo>,
}

impl DeclarativeTermRegion {
    pub(crate) fn new(
        path: SynNodeRegionPath,
        term_symbol_region: SymbolDeclarativeTermRegion,
        expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprDeclarativeTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolDeclarativeTypeInfo>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
        }
    }

    pub fn term_symbol_region(&self) -> &SymbolDeclarativeTermRegion {
        &self.term_symbol_region
    }

    pub fn current_syn_symbol_signature(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.term_symbol_region
            .current_parameter_symbol_signature(current_syn_symbol_idx)
    }

    pub fn expr_term(&self, expr: SynExprIdx) -> DeclarativeTermResultBorrowed2<DeclarativeTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }
}
