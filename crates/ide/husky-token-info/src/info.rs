use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::{EntityPath, ItemPath};
use husky_entity_syn_tree::{OnceUseRuleIdx, OnceUseRuleState, UseExprIdx};
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{
    SynCurrentSymbolIdx, SynCurrentSymbolKind, SynExprRegion, SynInheritedSymbolIdx,
    SynInheritedSymbolKind, SynPrincipalEntityPathExprIdx,
};

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb)]
pub struct TokenInfo {
    src: TokenInfoSource,
    data: TokenInfoData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenInfoDb)]
#[enum_class::from_variants]
pub enum TokenInfoSource {
    UseExpr(UseExprIdx),
    SemaExpr(SemaExprIdx),
    SynPrincipalEntityPathExpr(SynPrincipalEntityPathExprIdx),
    SynCurrentSymbol(SynCurrentSymbolIdx),
    AstIdentifiable,
}

impl TokenInfo {
    pub fn new(src: impl Into<TokenInfoSource>, data: TokenInfoData) -> Self {
        Self {
            src: src.into(),
            data,
        }
    }

    pub fn src(&self) -> TokenInfoSource {
        self.src
    }

    pub fn data(&self) -> &TokenInfoData {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb)]
pub enum TokenInfoData {
    Entity(EntityPath),
    EntityNode(ItemSynNodePath, EntityKind),
    InheritedSymbol {
        inherited_symbol_idx: SynInheritedSymbolIdx,
        inherited_symbol_kind: SynInheritedSymbolKind,
        syn_expr_region: ExprRegionLeash,
    },
    CurrentSymbol {
        current_symbol_idx: SynCurrentSymbolIdx,
        current_symbol_kind: SynCurrentSymbolKind,
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
    Unreachable,
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
