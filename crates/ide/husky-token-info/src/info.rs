use crate::*;
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_entity_tree::{UseExprIdx, UseExprRuleIdx, UseExprRuleState};
use husky_expr::{
    CurrentSymbolIdx, CurrentSymbolKind, ExprRegion, InheritedSymbolIdx, InheritedSymbolKind,
};

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TokenInfoDb)]
pub enum TokenInfo {
    None,
    Entity(EntityPath),
    EntityNode(EntityNodePath, EntityKind),
    InheritedSymbol {
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
        expr_region: ExprRegionLeash,
    },
    CurrentSymbol {
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
        expr_region: ExprRegionLeash,
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
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    SubmoduleIdent,
}

/// the purpose is to avoid extra debug with db in expr region
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprRegionLeash(ExprRegion);

impl From<ExprRegion> for ExprRegionLeash {
    fn from(value: ExprRegion) -> Self {
        ExprRegionLeash(value)
    }
}
impl From<ExprRegionLeash> for ExprRegion {
    fn from(value: ExprRegionLeash) -> Self {
        value.0
    }
}

impl std::ops::Deref for ExprRegionLeash {
    type Target = ExprRegion;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for ExprRegionLeash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}

impl<Db: ?Sized> salsa::DebugWithDb<Db> for ExprRegionLeash {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
