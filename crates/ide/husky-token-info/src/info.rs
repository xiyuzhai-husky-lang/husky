use crate::*;
use husky_entity_kind::EntityKind;

use husky_entity_path::{EntityPath, PrincipalEntityPath};

use husky_entity_syn_tree::{OnceUseRuleIdx, OnceUseRuleState, UseExprIdx};
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{
    CurrentSynSymbolIdx, CurrentSynSymbolKind, InheritedSynSymbolIdx, InheritedSynSymbolKind,
    SynExprRegion, SynPatternExprIdx, SynPrincipalEntityPathExprIdx,
};
#[cfg(feature = "protocol_support")]
use husky_token_protocol::*;
use salsa::Db;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TokenInfoDb, jar = TokenInfoJar)]
pub struct TokenInfo {
    src: TokenInfoSource,
    data: TokenInfoData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TokenInfoDb, jar = TokenInfoJar)]
#[enum_class::from_variants]
pub enum TokenInfoSource {
    UseExpr(UseExprIdx),
    SemaExpr(SemaExprIdx),
    SynPrincipalEntityPathExpr(SynPrincipalEntityPathExprIdx, PrincipalEntityPath),
    PatternExpr(SynPatternExprIdx),
    // todo: add #[skip] attribute
    TemplateParameter(CurrentSynSymbolIdx),
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
#[salsa::debug_with_db(db = TokenInfoDb, jar = TokenInfoJar)]
pub enum TokenInfoData {
    Entity(EntityPath),
    EntityNode(ItemSynNodePath, EntityKind),
    InheritedSynSymbol {
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
        syn_expr_region: ExprRegionLeash,
    },
    CurrentSynSymbol {
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
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
    SemaPrefixTypeOpr,
    CallPar,
}

#[cfg(feature = "protocol_support")]
impl TokenInfoData {
    pub fn token_class(&self, db: &dyn TokenInfoDb) -> TokenClass {
        match self {
            TokenInfoData::Entity(path) => path.item_kind(db).class().into(),
            TokenInfoData::EntityNode(_path, item_kind) => item_kind.class().into(),
            TokenInfoData::CurrentSynSymbol {
                current_syn_symbol_kind,
                ..
            } => match current_syn_symbol_kind {
                CurrentSynSymbolKind::LetVariable { .. }
                | CurrentSynSymbolKind::BeVariable { .. }
                | CurrentSynSymbolKind::CaseVariable { .. } => TokenClass::Variable,
                CurrentSynSymbolKind::ExplicitRegularParameter { .. } => TokenClass::Parameter,
                CurrentSynSymbolKind::LoopVariable(_) => TokenClass::LoopVariable,
                CurrentSynSymbolKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                CurrentSynSymbolKind::ExplicitVariadicParameter { .. } => TokenClass::Parameter,
                CurrentSynSymbolKind::FieldVariable { .. } => TokenClass::Variable,
            },
            // TokenProtocol::Variable,
            TokenInfoData::InheritedSynSymbol {
                inherited_syn_symbol_kind,
                ..
            } => match inherited_syn_symbol_kind {
                InheritedSynSymbolKind::ParenateParameter { .. } => TokenClass::Parameter,
                InheritedSynSymbolKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                InheritedSynSymbolKind::FieldVariable { .. } => TokenClass::Variable,
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
            TokenInfoData::UseExprStar => TokenClass::Punctuation,
            TokenInfoData::HtmlFunctionIdent => TokenClass::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => TokenClass::HtmlPropertyIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                TokenClass::TypeEntity
            }
            TokenInfoData::Todo => TokenClass::Todo,
            TokenInfoData::Unreachable => TokenClass::Unreachable,
            TokenInfoData::SemaPrefixTypeOpr => TokenClass::TypeEntity,
            TokenInfoData::CallPar => TokenClass::Punctuation,
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

impl salsa::DebugWithDb for ExprRegionLeash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, _db: &::salsa::Db) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}
