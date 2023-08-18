use crate::*;
use husky_entity_path::{EntityPath, ItemPath};
use husky_entity_syn_tree::{OnceUseRuleIdx, OnceUseRuleState, UseExprIdx};
use husky_entity_taxonomy::EntityKind;
use husky_syn_expr::{
    CurrentSynSymbolIdx, CurrentSynSymbolKind, InheritedSynSymbolIdx, InheritedSynSymbolKind,
    SynExprRegion,
};

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb)]
pub enum TokenInfo {
    None,
    Entity(EntityPath),
    EntityNode(ItemSynNodePath, EntityKind),
    InheritedSymbol {
        inherited_symbol_idx: InheritedSynSymbolIdx,
        inherited_symbol_kind: InheritedSynSymbolKind,
        syn_expr_region: ExprRegionLeash,
    },
    CurrentSymbol {
        current_symbol_idx: CurrentSynSymbolIdx,
        current_symbol_kind: CurrentSynSymbolKind,
        syn_expr_region: ExprRegionLeash,
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
        rule_idx: OnceUseRuleIdx,
        state: OnceUseRuleState,
    },
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    SubmoduleIdent,
    UnitLeftParenthesis,
    UnitRightParenthesis,
    Todo,
}

/// the purpose is to avoid extra debug with db in expr region
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprRegionLeash(SynExprRegion);

impl From<SynExprRegion> for ExprRegionLeash {
    fn from(value: SynExprRegion) -> Self {
        ExprRegionLeash(value)
    }
}
impl From<ExprRegionLeash> for SynExprRegion {
    fn from(value: ExprRegionLeash) -> Self {
        value.0
    }
}

impl std::ops::Deref for ExprRegionLeash {
    type Target = SynExprRegion;

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
