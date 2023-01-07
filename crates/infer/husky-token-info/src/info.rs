use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_expr::{ExprSheet, InheritedSymbolIdx, LocalSymbolIdx, LocalSymbolKind};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    InheritedSymbol {
        inherited_symbol_idx: InheritedSymbolIdx,
        expr_sheet: ExprSheet,
    },
    LocalSymbol {
        local_symbol_idx: LocalSymbolIdx,
        expr_sheet: ExprSheet,
        local_symbol_kind: LocalSymbolKind,
    },
    Field,
    Method,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
