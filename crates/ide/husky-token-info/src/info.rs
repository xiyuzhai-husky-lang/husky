use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_kind::*;
use husky_entity_path::{EntityPath, ItemPath};
use husky_entity_syn_tree::{OnceUseRuleIdx, OnceUseRuleState, UseExprIdx};
use husky_sema_expr::SemaExprIdx;
#[cfg(feature = "semantic_token_support")]
use husky_semantic_token_kind::*;
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

#[cfg(feature = "semantic_token_support")]
impl TokenInfoData {
    pub fn semantic_token_kind(&self, db: &dyn TokenInfoDb) -> SemanticTokenKind {
        match self {
            TokenInfoData::Entity(path) => SemanticTokenKind::Entity(path.item_kind(db)),
            TokenInfoData::EntityNode(path, item_kind) => SemanticTokenKind::Entity(*item_kind),
            TokenInfoData::CurrentSymbol {
                current_symbol_kind,
                ..
            } => match current_symbol_kind {
                SynCurrentSymbolKind::LetVariable { .. }
                | SynCurrentSymbolKind::BeVariable { .. }
                | SynCurrentSymbolKind::CaseVariable { .. } => SemanticTokenKind::Variable,
                SynCurrentSymbolKind::ExplicitRegularParameter { .. } => {
                    SemanticTokenKind::Parameter
                }
                SynCurrentSymbolKind::FrameVariable(_) => SemanticTokenKind::FrameVariable,
                SynCurrentSymbolKind::ImplicitParameter { .. } => {
                    SemanticTokenKind::ImplicitParameter
                }
                SynCurrentSymbolKind::ExplicitVariadicParameter { .. } => {
                    SemanticTokenKind::Parameter
                }
                SynCurrentSymbolKind::FieldVariable { .. } => SemanticTokenKind::Variable,
            },
            // SemanticTokenKind::Variable,
            TokenInfoData::InheritedSymbol {
                inherited_symbol_kind,
                ..
            } => match inherited_symbol_kind {
                SynInheritedSymbolKind::ParenateParameter { .. } => SemanticTokenKind::Parameter,
                SynInheritedSymbolKind::TemplateParameter { .. } => {
                    SemanticTokenKind::ImplicitParameter
                }
                SynInheritedSymbolKind::FieldVariable { .. } => SemanticTokenKind::Variable,
            },
            TokenInfoData::SelfType => SemanticTokenKind::SelfType,
            TokenInfoData::SelfValue => SemanticTokenKind::SelfValue,
            // SemanticTokenKind::Variable,
            TokenInfoData::Field => SemanticTokenKind::Field,
            TokenInfoData::Method => SemanticTokenKind::Method,
            TokenInfoData::BoxColon
            | TokenInfoData::VecFunctorBoxPrefix
            | TokenInfoData::ArrayFunctorBoxPrefix => {
                SemanticTokenKind::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::UseExpr { state, .. } => match state {
                OnceUseRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => SemanticTokenKind::Entity(original_symbol.path(db).item_kind(db)),
                OnceUseRuleState::Resolved {
                    original_symbol: None,
                } => todo!(),
                OnceUseRuleState::Unresolved => todo!(),
                OnceUseRuleState::Erroneous => SemanticTokenKind::Error,
            },
            TokenInfoData::UseExprStar => SemanticTokenKind::Special,
            TokenInfoData::HtmlFunctionIdent => SemanticTokenKind::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => SemanticTokenKind::HtmlPropertyIdent,
            TokenInfoData::SubmoduleIdent => SemanticTokenKind::SubmoduleIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                SemanticTokenKind::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::Todo => SemanticTokenKind::Todo,
            TokenInfoData::Unreachable => SemanticTokenKind::Unreachable,
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
