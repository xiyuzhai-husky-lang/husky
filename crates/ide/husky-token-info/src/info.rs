use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::{EntityPath, PrincipalEntityPath};
use husky_entity_tree::{OnceUseRuleIdx, UseExprIdx, UseOneRuleState};
use husky_sem_expr::SemaExprIdx;
use husky_syn_expr::{
    entity_path::SynPrincipalEntityPathSynExprIdx, CurrentVariableIdx, CurrentVariableKind,
    InheritedSymbolicVariableIdx, InheritedVariableKind, SynExprRegion, SynPatternIdx,
};
#[cfg(feature = "protocol_support")]
use husky_token_protocol::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct TokenInfo {
    src: TokenInfoSource,
    data: TokenInfoData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TokenInfoSource {
    UseExpr(UseExprIdx),
    SemaExpr(SemaExprIdx),
    SynPrincipalEntityPathExpr(SynPrincipalEntityPathSynExprIdx, PrincipalEntityPath),
    PatternExpr(SynPatternIdx),
    // todo: add #[skip] attribute
    TemplateParameter(CurrentVariableIdx),
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

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfoData {
    Entity(EntityPath),
    EntityNode(ItemSynNodePath, EntityKind),
    InheritedSynSymbol {
        inherited_syn_symbol_idx: InheritedSymbolicVariableIdx,
        inherited_syn_symbol_kind: InheritedVariableKind,
        syn_expr_region: ExprRegionLeash,
    },
    CurrentSynSymbol {
        current_syn_symbol_idx: CurrentVariableIdx,
        current_syn_symbol_kind: CurrentVariableKind,
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
        state: UseOneRuleState,
    },
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    UnitLeftParenthesis,
    UnitRightParenthesis,
    Todo,
    Unreachable,
    SemaPrefixTypeOpr,
    CallPar,
    NestedBlockCurl,
    ClosureVert,
    ClosureLightArrow,
    ClosureEq,
}

#[cfg(feature = "protocol_support")]
impl TokenInfoData {
    pub fn token_class(&self, db: &::salsa::Db) -> TokenClass {
        match self {
            TokenInfoData::Entity(path) => path.item_kind(db).class().into(),
            TokenInfoData::EntityNode(_path, item_kind) => item_kind.class().into(),
            TokenInfoData::CurrentSynSymbol {
                current_syn_symbol_kind,
                ..
            } => match current_syn_symbol_kind {
                CurrentVariableKind::LetVariable { .. }
                | CurrentVariableKind::BeVariable { .. }
                | CurrentVariableKind::CaseVariable { .. } => TokenClass::Variable,
                CurrentVariableKind::SimpleParenateParameter { .. }
                | CurrentVariableKind::VariadicParenateParameter { .. }
                | CurrentVariableKind::SimpleClosureParameter { .. } => TokenClass::Parameter,
                CurrentVariableKind::LoopVariable(_) => TokenClass::LoopVariable,
                CurrentVariableKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                CurrentVariableKind::FieldVariable { .. } => TokenClass::Variable,
            },
            // TokenProtocol::Variable,
            TokenInfoData::InheritedSynSymbol {
                inherited_syn_symbol_kind,
                ..
            } => match inherited_syn_symbol_kind {
                InheritedVariableKind::Parenate { .. } => TokenClass::Parameter,
                InheritedVariableKind::Template { .. } => TokenClass::ImplicitParameter,
                InheritedVariableKind::SelfField { .. } => TokenClass::Variable,
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
                UseOneRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => original_symbol
                    .principal_entity_path(db)
                    .item_kind(db)
                    .class()
                    .into(),
                UseOneRuleState::Resolved {
                    original_symbol: None,
                } => todo!(),
                UseOneRuleState::Unresolved => todo!(),
                UseOneRuleState::Erroneous => TokenClass::Error,
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
            TokenInfoData::NestedBlockCurl => TokenClass::Punctuation,
            TokenInfoData::ClosureVert => TokenClass::Punctuation,
            TokenInfoData::ClosureLightArrow => TokenClass::Punctuation,
            TokenInfoData::ClosureEq => TokenClass::Punctuation,
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
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}
