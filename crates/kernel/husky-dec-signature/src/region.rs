// todo: move this to husky-dec-term
pub mod error;
pub mod variable;

pub(crate) use engine::*;

use crate::*;
use husky_entity_tree::region_path::SynNodeRegionPath;
use husky_syn_expr::{
    expr::{SynExprIdx, SynExprMap},
    pattern::{SynPatternMap, SynPatternSymbolMap},
    variable::CurrentVariableIdx,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
///
/// should contains term information enough for generating declarations
#[derive(Debug, PartialEq, Eq)]
pub struct SynExprDecTermRegion {
    path: SynNodeRegionPath,
    symbolic_variable_region: DecSymbolicVariableRegion,
    expr_terms: SynExprMap<SynExprDecTermResult<DecTerm>>,
    pattern_expr_ty_infos: SynPatternMap<PatternDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<DecPatternVariableTypeInfo>,
}

impl SynExprDecTermRegion {
    pub(crate) fn new(
        path: SynNodeRegionPath,
        symbolic_variable_region: DecSymbolicVariableRegion,
        expr_terms: SynExprMap<SynExprDecTermResult<DecTerm>>,
        pattern_expr_ty_infos: SynPatternMap<PatternDeclarativeTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<DecPatternVariableTypeInfo>,
    ) -> Self {
        Self {
            path,
            symbolic_variable_region,
            expr_terms,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
        }
    }

    pub fn symbolic_variable_region(&self) -> &DecSymbolicVariableRegion {
        &self.symbolic_variable_region
    }

    pub fn current_variable_signature(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<DecSymbolicVariableSignature> {
        self.symbolic_variable_region
            .current_parameter_variable_signature(current_variable_idx)
    }

    pub fn expr_term(&self, expr: SynExprIdx) -> SynExprDecTermResultRef<DecTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }
}
