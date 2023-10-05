use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_kind::*;
use husky_entity_path::{EntityPath, ItemPath};
#[cfg(feature = "protocol_support")]
use husky_entity_protocol::*;
use husky_entity_syn_tree::{OnceUseRuleIdx, OnceUseRuleState, UseExprIdx};
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{
    SynCurrentSymbolIdx, SynCurrentSymbolKind, SynExprRegion, SynInheritedSymbolIdx,
    SynInheritedSymbolKind, SynPrincipalEntityPathExprIdx,
};
#[cfg(feature = "protocol_support")]
use husky_token_protocol::*;

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
    VecFunctorBoxPrefix,
    ArrayFunctorBoxPrefix,
    UseExprStar,
    UseExpr {
        use_expr_idx: UseExprIdx,
        rule_idx: OnceUseRuleIdx,
        state: OnceUseRuleState,
    },
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    UnitLeftParenthesis,
    UnitRightParenthesis,
    Todo,
    Unreachable,
}

#[cfg(feature = "protocol_support")]
impl TokenInfoData {
    pub fn semantic_token_kind(&self, db: &dyn TokenInfoDb) -> TokenClass {
        match self {
            TokenInfoData::Entity(path) => path.item_kind(db).class().into(),
            TokenInfoData::EntityNode(path, item_kind) => item_kind.class().into(),
            TokenInfoData::CurrentSymbol {
                current_symbol_kind,
                ..
            } => match current_symbol_kind {
                SynCurrentSymbolKind::LetVariable { .. }
                | SynCurrentSymbolKind::BeVariable { .. }
                | SynCurrentSymbolKind::CaseVariable { .. } => TokenClass::Variable,
                SynCurrentSymbolKind::ExplicitRegularParameter { .. } => TokenClass::Parameter,
                SynCurrentSymbolKind::FrameVariable(_) => TokenClass::FrameVariable,
                SynCurrentSymbolKind::ImplicitParameter { .. } => TokenClass::ImplicitParameter,
                SynCurrentSymbolKind::ExplicitVariadicParameter { .. } => TokenClass::Parameter,
                SynCurrentSymbolKind::FieldVariable { .. } => TokenClass::Variable,
            },
            // TokenProtocol::Variable,
            TokenInfoData::InheritedSymbol {
                inherited_symbol_kind,
                ..
            } => match inherited_symbol_kind {
                SynInheritedSymbolKind::ParenateParameter { .. } => TokenClass::Parameter,
                SynInheritedSymbolKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                SynInheritedSymbolKind::FieldVariable { .. } => TokenClass::Variable,
            },
            TokenInfoData::SelfType => TokenClass::SelfType,
            TokenInfoData::SelfValue => TokenClass::SelfValue,
            // TokenProtocol::Variable,
            TokenInfoData::Field => TokenClass::Field,
            TokenInfoData::Method => TokenClass::Method,
            TokenInfoData::BoxColon
            | TokenInfoData::VecFunctorBoxPrefix
            | TokenInfoData::ArrayFunctorBoxPrefix => TokenClass::TypeEntity,
            TokenInfoData::UseExpr { state, .. } => match state {
                OnceUseRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => original_symbol.path(db).item_kind(db).class().into(),
                OnceUseRuleState::Resolved {
                    original_symbol: None,
                } => todo!(),
                OnceUseRuleState::Unresolved => todo!(),
                OnceUseRuleState::Erroneous => TokenClass::Error,
            },
            TokenInfoData::UseExprStar => TokenClass::Special,
            TokenInfoData::HtmlFunctionIdent => TokenClass::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => TokenClass::HtmlPropertyIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                TokenClass::TypeEntity
            }
            TokenInfoData::Todo => TokenClass::Todo,
            TokenInfoData::Unreachable => TokenClass::Unreachable,
        }
    }
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
