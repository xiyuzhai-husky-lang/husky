// todo: move this to husky-declarative-term
mod error;
mod symbol;

pub use self::error::*;
pub use self::symbol::*;

pub(crate) use engine::*;

use crate::*;

use husky_entity_tree::RegionPath;
use husky_syn_expr::{
    AllowSelfType, CurrentSynSymbolIdx, InheritedSymbolIdx, ParentSymbolIdx, PatternSymbolMap,
    PatternSynExprIdx, PatternSynExprMap, SynExprIdx, SynExprMap, SynExprRegion, SynSymbolRegion,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
///
/// should contains term information enough for generating declarations
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermRegion {
    path: RegionPath,
    term_symbol_region: SymbolDeclarativeTermRegion,
    expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
    pattern_expr_ty_infos: PatternSynExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
}

impl DeclarativeTermRegion {
    pub(crate) fn new(
        path: RegionPath,
        term_symbol_region: SymbolDeclarativeTermRegion,
        expr_terms: SynExprMap<DeclarativeTermResult2<DeclarativeTerm>>,
        pattern_expr_ty_infos: PatternSynExprMap<PatternExprDeclarativeTypeInfo>,
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

    pub fn term_symbol_region(&self) -> &SymbolDeclarativeTermRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_signature(
        &self,
        current_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.term_symbol_region
            .current_symbol_signature(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: SynExprIdx) -> DeclarativeTermResultBorrowed2<DeclarativeTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }
}
