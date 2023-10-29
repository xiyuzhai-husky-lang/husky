mod context_mut;
mod map;
mod ordered_map;
mod region;

pub use self::context_mut::*;
pub use self::map::*;
pub use self::ordered_map::*;
pub use self::region::*;

use crate::*;
use husky_entity_syn_tree::{CratePrelude, ModuleSymbolContext, PreludeResult};
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::{IsStreamParser, TryParseFromStream};
use vec_like::SmallVecSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(SynInheritedSymbolIdx, SynInheritedSymbolKind),
    Current(SynCurrentSymbolIdx, SynCurrentSymbolKind),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum ImplicitParameterSymbol {
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Type {
        ident_token: IdentRegionalToken,
    },
    Const {},
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynInheritedSymbol {
    parent_symbol_idx: ParentSynSymbolIdx,
    modifier: SymbolModifier,
    kind: SynInheritedSymbolKind,
}

impl SynInheritedSymbol {
    pub fn kind(&self) -> SynInheritedSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSynSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.kind {
            SynInheritedSymbolKind::TemplateParameter(kind) => match kind {
                InheritedTemplateParameterSynSymbol::Lifetime { .. }
                | InheritedTemplateParameterSynSymbol::Place { .. } => None,
                InheritedTemplateParameterSynSymbol::Type { ident }
                | InheritedTemplateParameterSynSymbol::Constant { ident } => Some(ident),
            },
            SynInheritedSymbolKind::ParenateParameter { ident } => Some(ident),
            SynInheritedSymbolKind::FieldVariable { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynInheritedSymbolKind {
    TemplateParameter(InheritedTemplateParameterSynSymbol),
    ParenateParameter { ident: Ident },
    FieldVariable { ident: Ident },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum InheritedTemplateParameterSynSymbol {
    Lifetime { label: Label },
    Place { label: Label },
    Type { ident: Ident },
    Constant { ident: Ident },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynCurrentSymbol {
    modifier: SymbolModifier,
    access_start: RegionalTokenIdx,
    /// this is none only for lambda variable
    access_end: Option<RegionalTokenIdxRangeEnd>,
    variant: SynCurrentSymbolVariant,
}

impl SynCurrentSymbol {
    pub fn new(
        pattern_expr_region: &SynPatternExprRegion,
        access_start: RegionalTokenIdx,
        access_end: Option<RegionalTokenIdxRangeEnd>,
        variant: SynCurrentSymbolVariant,
    ) -> Self {
        Self {
            modifier: variant.symbol_modifier(pattern_expr_region),
            access_start,
            access_end,
            variant,
        }
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn kind(&self) -> SynCurrentSymbolKind {
        self.variant.kind()
    }

    pub fn variant(&self) -> &SynCurrentSymbolVariant {
        &self.variant
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.variant {
            SynCurrentSymbolVariant::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | SynCurrentSymbolVariant::ParenateVariadicParameter { ident_token, .. }
            | SynCurrentSymbolVariant::FieldVariable { ident_token } => Some(ident_token.ident()),
            SynCurrentSymbolVariant::ParenateRegularParameter { ident, .. }
            | SynCurrentSymbolVariant::LetVariable { ident, .. }
            | SynCurrentSymbolVariant::BeVariable { ident, .. }
            | SynCurrentSymbolVariant::CaseVariable { ident, .. }
            | SynCurrentSymbolVariant::FrameVariable { ident, .. } => Some(ident),
            SynCurrentSymbolVariant::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Lifetime { .. }
                    | CurrentTemplateParameterSynSymbolVariant::Place { .. },
                ..
            } => None,
            SynCurrentSymbolVariant::SelfType | SynCurrentSymbolVariant::SelfValue { .. } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynCurrentSymbolKind {
    TemplateParameter {
        template_parameter_kind: CurrentImplicitParameterSynSymbolKind,
    },
    ExplicitRegularParameter {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    ExplicitVariadicParameter {
        ident_token: IdentRegionalToken,
    },
    LetVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    BeVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    CaseVariable {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    FieldVariable {
        ident_token: IdentRegionalToken,
    },
    FrameVariable(SynExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum CurrentImplicitParameterSynSymbolKind {
    Type {
        ident_token: IdentRegionalToken,
    },
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Place {
        label_token: PlaceLabelRegionalToken,
    },
    Constant {
        ident_token: IdentRegionalToken,
    },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynCurrentSymbolVariant {
    TemplateParameter {
        syn_attrs: TemplateParameterSynAttrs,
        annotated_variance_token: Option<VarianceRegionalToken>,
        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant,
    },
    SelfType,
    SelfValue {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
    },
    ParenateRegularParameter {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    ParenateVariadicParameter {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
    LetVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    BeVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    CaseVariable {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    FieldVariable {
        ident_token: IdentRegionalToken,
    },
    FrameVariable {
        ident: Ident,
        expr_idx: SynExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct TemplateParameterSynAttrs {
    syn_attrs: SmallVec<[TemplateSymbolSynAttr; 1]>,
}

impl std::ops::Deref for TemplateParameterSynAttrs {
    type Target = [TemplateSymbolSynAttr];

    fn deref(&self) -> &Self::Target {
        &self.syn_attrs
    }
}

impl<'a, C> TryParseFromStream<SynExprParser<'a, C>> for TemplateParameterSynAttrs
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_from_stream(sp: &mut SynExprParser<'a, C>) -> Result<Self, Self::Error> {
        let mut syn_attrs: SmallVec<[TemplateSymbolSynAttr; 1]> = smallvec::smallvec![];
        while let Some(_) = sp.try_parse_option::<PoundRegionalToken>()? {
            todo!()
        }
        Ok(TemplateParameterSynAttrs { syn_attrs })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateSymbolSynAttr {
    Phantom(PoundRegionalToken, PhantomRegionalToken),
}

impl SynCurrentSymbolVariant {
    fn symbol_modifier(&self, pattern_expr_region: &SynPatternExprRegion) -> SymbolModifier {
        match self {
            SynCurrentSymbolVariant::TemplateParameter {
                template_parameter_variant,
                ..
            } => SymbolModifier::Const,
            SynCurrentSymbolVariant::ParenateRegularParameter {
                pattern_symbol_idx, ..
            }
            | SynCurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            }
            | SynCurrentSymbolVariant::BeVariable {
                pattern_symbol_idx, ..
            }
            | SynCurrentSymbolVariant::CaseVariable {
                pattern_symbol_idx, ..
            } => pattern_expr_region.pattern_symbol_modifier(*pattern_symbol_idx),
            SynCurrentSymbolVariant::ParenateVariadicParameter {
                symbol_modifier_keyword_group,
                ..
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            SynCurrentSymbolVariant::FrameVariable { ident, expr_idx } => SymbolModifier::None,
            SynCurrentSymbolVariant::SelfType => SymbolModifier::Const,
            SynCurrentSymbolVariant::SelfValue {
                symbol_modifier_keyword_group,
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            SynCurrentSymbolVariant::FieldVariable { ident_token } => SymbolModifier::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
#[non_exhaustive]
pub enum CurrentTemplateParameterSynSymbolVariant {
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Place {
        label_token: PlaceLabelRegionalToken,
    },
    Type {
        ident_token: IdentRegionalToken,
    },
    Constant {
        ident_token: IdentRegionalToken,
        ty_expr_idx: SynExprIdx,
    },
}

impl CurrentTemplateParameterSynSymbolVariant {
    fn bequeath(&self) -> InheritedTemplateParameterSynSymbol {
        match self {
            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                InheritedTemplateParameterSynSymbol::Lifetime {
                    label: label_token.label(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                InheritedTemplateParameterSynSymbol::Place {
                    label: label_token.label(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                InheritedTemplateParameterSynSymbol::Type {
                    ident: ident_token.ident(),
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Constant {
                ident_token,
                ty_expr_idx,
            } => InheritedTemplateParameterSynSymbol::Constant {
                ident: ident_token.ident(),
            },
        }
    }
}

impl SynCurrentSymbolVariant {
    pub fn kind(&self) -> SynCurrentSymbolKind {
        match self {
            SynCurrentSymbolVariant::TemplateParameter {
                template_parameter_variant,
                ..
            } => SynCurrentSymbolKind::TemplateParameter {
                template_parameter_kind: template_parameter_variant.kind(),
            },
            SynCurrentSymbolVariant::ParenateRegularParameter {
                pattern_symbol_idx, ..
            } => SynCurrentSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            SynCurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => SynCurrentSymbolKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            SynCurrentSymbolVariant::BeVariable {
                pattern_symbol_idx, ..
            } => SynCurrentSymbolKind::BeVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            SynCurrentSymbolVariant::CaseVariable {
                pattern_symbol_idx, ..
            } => SynCurrentSymbolKind::CaseVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            SynCurrentSymbolVariant::FrameVariable { expr_idx, .. } => {
                SynCurrentSymbolKind::FrameVariable(*expr_idx)
            }
            SynCurrentSymbolVariant::ParenateVariadicParameter { ident_token, .. } => {
                SynCurrentSymbolKind::ExplicitVariadicParameter {
                    ident_token: *ident_token,
                }
            }
            SynCurrentSymbolVariant::SelfType => todo!(),
            SynCurrentSymbolVariant::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            SynCurrentSymbolVariant::FieldVariable { ident_token } => {
                SynCurrentSymbolKind::FieldVariable {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

impl CurrentTemplateParameterSynSymbolVariant {
    fn kind(&self) -> CurrentImplicitParameterSynSymbolKind {
        match self {
            CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                CurrentImplicitParameterSynSymbolKind::Type {
                    ident_token: *ident_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                CurrentImplicitParameterSynSymbolKind::Lifetime {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                CurrentImplicitParameterSynSymbolKind::Place {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. } => {
                CurrentImplicitParameterSynSymbolKind::Constant {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

pub type SynInheritedSymbolArena = Arena<SynInheritedSymbol>;
pub type SynInheritedSymbolIdx = ArenaIdx<SynInheritedSymbol>;
pub type SynInheritedSymbolIdxRange = ArenaIdxRange<SynInheritedSymbol>;
pub(crate) type SynInheritedSymbolMap<V> = ArenaMap<SynInheritedSymbol, V>;
pub(crate) type SynInheritedSymbolOrderedMap<V> = ArenaOrderedMap<SynInheritedSymbol, V>;

pub type SynCurrentSymbolArena = Arena<SynCurrentSymbol>;
pub type SynCurrentSymbolIdx = ArenaIdx<SynCurrentSymbol>;
pub type SynCurrentSymbolIdxRange = ArenaIdxRange<SynCurrentSymbol>;
pub(crate) type SynCurrentSymbolMap<V> = ArenaMap<SynCurrentSymbol, V>;
pub(crate) type SynCurrentSymbolOrderedMap<V> = ArenaOrderedMap<SynCurrentSymbol, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentSynSymbolIdx {
    Inherited(SynInheritedSymbolIdx),
    Current(SynCurrentSymbolIdx),
}

impl From<SynInheritedSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: SynInheritedSymbolIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<SynCurrentSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: SynCurrentSymbolIdx) -> Self {
        Self::Current(v)
    }
}
