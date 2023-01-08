use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_expr::{
    ExprSheet, InheritedSymbolIdx, InheritedSymbolKind, LocalSymbolIdx, LocalSymbolKind,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    InheritedSymbol {
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
        expr_sheet: ExprSheet,
    },
    LocalSymbol {
        local_symbol_idx: LocalSymbolIdx,
        local_symbol_kind: LocalSymbolKind,
        expr_sheet: ExprSheet,
    },
    Field,
    Method,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
