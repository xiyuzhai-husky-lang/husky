use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_entity_tree::{UseExprIdx, UseExprRuleIdx, UseExprRuleState};
use husky_expr::{
    CurrentSymbolIdx, CurrentSymbolKind, ExprRegion, InheritedSymbolIdx, InheritedSymbolKind,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(Option<EntityPath>, Option<EntityKind>),
    InheritedSymbol {
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
        expr_region: ExprRegion,
    },
    CurrentSymbol {
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
        expr_region: ExprRegion,
    },
    SelfType,
    SelfValue,
    Field,
    Method,
    BoxColon,
    BoxPrefix,
    UseExprStar,
    UseExpr {
        use_expr_idx: UseExprIdx,
        rule_idx: UseExprRuleIdx,
        state: UseExprRuleState,
    },
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
