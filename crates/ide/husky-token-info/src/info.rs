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
    SubmoduleIdent,
    UnitLeftParenthesis,
    UnitRightParenthesis,
    Todo,
    Unreachable,
}

#[cfg(feature = "protocol_support")]
impl TokenInfoData {
    pub fn semantic_token_kind(&self, db: &dyn TokenInfoDb) -> TokenKindProtocol {
        match self {
            TokenInfoData::Entity(path) => TokenKindProtocol::Entity(path.item_kind(db).protocol()),
            TokenInfoData::EntityNode(path, item_kind) => {
                TokenKindProtocol::Entity(item_kind.protocol())
            }
            TokenInfoData::CurrentSymbol {
                current_symbol_kind,
                ..
            } => match current_symbol_kind {
                SynCurrentSymbolKind::LetVariable { .. }
                | SynCurrentSymbolKind::BeVariable { .. }
                | SynCurrentSymbolKind::CaseVariable { .. } => TokenKindProtocol::Variable,
                SynCurrentSymbolKind::ExplicitRegularParameter { .. } => {
                    TokenKindProtocol::Parameter
                }
                SynCurrentSymbolKind::FrameVariable(_) => TokenKindProtocol::FrameVariable,
                SynCurrentSymbolKind::ImplicitParameter { .. } => {
                    TokenKindProtocol::ImplicitParameter
                }
                SynCurrentSymbolKind::ExplicitVariadicParameter { .. } => {
                    TokenKindProtocol::Parameter
                }
                SynCurrentSymbolKind::FieldVariable { .. } => TokenKindProtocol::Variable,
            },
            // TokenProtocol::Variable,
            TokenInfoData::InheritedSymbol {
                inherited_symbol_kind,
                ..
            } => match inherited_symbol_kind {
                SynInheritedSymbolKind::ParenateParameter { .. } => TokenKindProtocol::Parameter,
                SynInheritedSymbolKind::TemplateParameter { .. } => {
                    TokenKindProtocol::ImplicitParameter
                }
                SynInheritedSymbolKind::FieldVariable { .. } => TokenKindProtocol::Variable,
            },
            TokenInfoData::SelfType => TokenKindProtocol::SelfType,
            TokenInfoData::SelfValue => TokenKindProtocol::SelfValue,
            // TokenProtocol::Variable,
            TokenInfoData::Field => TokenKindProtocol::Field,
            TokenInfoData::Method => TokenKindProtocol::Method,
            TokenInfoData::BoxColon
            | TokenInfoData::VecFunctorBoxPrefix
            | TokenInfoData::ArrayFunctorBoxPrefix => {
                TokenKindProtocol::Entity(EntityKindSketch::Type)
            }
            TokenInfoData::UseExpr { state, .. } => match state {
                OnceUseRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => TokenKindProtocol::Entity(original_symbol.path(db).item_kind(db).protocol()),
                OnceUseRuleState::Resolved {
                    original_symbol: None,
                } => todo!(),
                OnceUseRuleState::Unresolved => todo!(),
                OnceUseRuleState::Erroneous => TokenKindProtocol::Error,
            },
            TokenInfoData::UseExprStar => TokenKindProtocol::Special,
            TokenInfoData::HtmlFunctionIdent => TokenKindProtocol::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => TokenKindProtocol::HtmlPropertyIdent,
            TokenInfoData::SubmoduleIdent => TokenKindProtocol::SubmoduleIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                TokenKindProtocol::Entity(EntityKindSketch::Type)
            }
            TokenInfoData::Todo => TokenKindProtocol::Todo,
            TokenInfoData::Unreachable => TokenKindProtocol::Unreachable,
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
