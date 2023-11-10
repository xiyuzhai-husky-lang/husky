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
use husky_print_utils::p;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::{IsStreamParser, TryParseFromStream};
use vec_like::SmallVecSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(InheritedSynSymbolIdx, InheritedSynSymbolKind),
    Current(CurrentSynSymbolIdx, CurrentSynSymbolKind),
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
pub struct InheritedSynSymbol {
    parent_symbol_idx: ParentSynSymbolIdx,
    modifier: SymbolModifier,
    kind: InheritedSynSymbolKind,
}

impl InheritedSynSymbol {
    pub fn kind(&self) -> InheritedSynSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSynSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.kind {
            InheritedSynSymbolKind::TemplateParameter(kind) => match kind {
                InheritedTemplateParameterSynSymbol::Lifetime { .. }
                | InheritedTemplateParameterSynSymbol::Place { .. } => None,
                InheritedTemplateParameterSynSymbol::Type { ident }
                | InheritedTemplateParameterSynSymbol::Constant { ident } => Some(ident),
            },
            InheritedSynSymbolKind::ParenateParameter { ident } => Some(ident),
            InheritedSynSymbolKind::FieldVariable { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum InheritedSynSymbolKind {
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
pub struct CurrentSynSymbol {
    modifier: SymbolModifier,
    access_start: RegionalTokenIdx,
    /// this is none only for lambda variable
    access_end: Option<RegionalTokenIdxRangeEnd>,
    data: CurrentSynSymbolData,
}

impl CurrentSynSymbol {
    pub fn new(
        pattern_expr_region: &SynPatternExprRegion,
        access_start: RegionalTokenIdx,
        access_end: Option<RegionalTokenIdxRangeEnd>,
        variant: CurrentSynSymbolData,
    ) -> Self {
        Self {
            modifier: variant.symbol_modifier(pattern_expr_region),
            access_start,
            access_end,
            data: variant,
        }
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn kind(&self) -> CurrentSynSymbolKind {
        self.data.kind()
    }

    pub fn data(&self) -> &CurrentSynSymbolData {
        &self.data
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.data {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | CurrentSynSymbolData::ParenateVariadicParameter { ident_token, .. }
            | CurrentSynSymbolData::FieldVariable { ident_token } => Some(ident_token.ident()),
            CurrentSynSymbolData::ParenateRegularParameter { ident, .. }
            | CurrentSynSymbolData::LetVariable { ident, .. }
            | CurrentSynSymbolData::BeVariable { ident, .. }
            | CurrentSynSymbolData::CaseVariable { ident, .. }
            | CurrentSynSymbolData::LoopVariable { ident, .. } => Some(ident),
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Lifetime { .. }
                    | CurrentTemplateParameterSynSymbolVariant::Place { .. },
                ..
            } => None,
            CurrentSynSymbolData::SelfType | CurrentSynSymbolData::SelfValue { .. } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum CurrentSynSymbolKind {
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
    LoopVariable(SynExprIdx),
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
pub enum CurrentSynSymbolData {
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
    LoopVariable {
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
        while let Some(pound) = sp.try_parse_option::<PoundRegionalToken>()? {
            let syn_attr = if let Some(phantom) = sp.try_parse_option::<PhantomRegionalToken>()? {
                TemplateSymbolSynAttr::Phantom(pound, phantom)
            } else {
                todo!()
            };
            syn_attrs.push(syn_attr)
        }
        Ok(TemplateParameterSynAttrs { syn_attrs })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateSymbolSynAttr {
    Phantom(PoundRegionalToken, PhantomRegionalToken),
}

impl CurrentSynSymbolData {
    fn symbol_modifier(&self, pattern_expr_region: &SynPatternExprRegion) -> SymbolModifier {
        match self {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant,
                ..
            } => SymbolModifier::Const,
            CurrentSynSymbolData::ParenateRegularParameter {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::LetVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::BeVariable {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::CaseVariable {
                pattern_symbol_idx, ..
            } => pattern_expr_region.pattern_symbol_modifier(*pattern_symbol_idx),
            CurrentSynSymbolData::ParenateVariadicParameter {
                symbol_modifier_keyword_group,
                ..
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            CurrentSynSymbolData::LoopVariable { ident, expr_idx } => SymbolModifier::None,
            CurrentSynSymbolData::SelfType => SymbolModifier::Const,
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            CurrentSynSymbolData::FieldVariable { ident_token } => SymbolModifier::None,
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

impl CurrentSynSymbolData {
    pub fn kind(&self) -> CurrentSynSymbolKind {
        match self {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant,
                ..
            } => CurrentSynSymbolKind::TemplateParameter {
                template_parameter_kind: template_parameter_variant.kind(),
            },
            CurrentSynSymbolData::ParenateRegularParameter {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolData::LetVariable {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolData::BeVariable {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::BeVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolData::CaseVariable {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::CaseVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolData::LoopVariable { expr_idx, .. } => {
                CurrentSynSymbolKind::LoopVariable(*expr_idx)
            }
            CurrentSynSymbolData::ParenateVariadicParameter { ident_token, .. } => {
                CurrentSynSymbolKind::ExplicitVariadicParameter {
                    ident_token: *ident_token,
                }
            }
            CurrentSynSymbolData::SelfType => todo!(),
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => todo!(),
            CurrentSynSymbolData::FieldVariable { ident_token } => {
                CurrentSynSymbolKind::FieldVariable {
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

pub type InheritedSynSymbolArena = Arena<InheritedSynSymbol>;
pub type InheritedSynSymbolIdx = ArenaIdx<InheritedSynSymbol>;
pub type InheritedSynSymbolIdxRange = ArenaIdxRange<InheritedSynSymbol>;
pub(crate) type InheritedSynSymbolMap<V> = ArenaMap<InheritedSynSymbol, V>;
pub(crate) type InheritedSynSymbolOrderedMap<V> = ArenaOrderedMap<InheritedSynSymbol, V>;

pub type CurrentSynSymbolArena = Arena<CurrentSynSymbol>;
pub type CurrentSynSymbolIdx = ArenaIdx<CurrentSynSymbol>;
pub type CurrentSynSymbolIdxRange = ArenaIdxRange<CurrentSynSymbol>;
pub(crate) type CurrentSynSymbolMap<V> = ArenaMap<CurrentSynSymbol, V>;
pub(crate) type CurrentSynSymbolOrderedMap<V> = ArenaOrderedMap<CurrentSynSymbol, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentSynSymbolIdx {
    Inherited(InheritedSynSymbolIdx),
    Current(CurrentSynSymbolIdx),
}

impl From<InheritedSynSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: InheritedSynSymbolIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<CurrentSynSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: CurrentSynSymbolIdx) -> Self {
        Self::Current(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SynSymbolMap<V> {
    inherited_syn_symbol_map: InheritedSynSymbolMap<V>,
    current_syn_symbol_map: CurrentSynSymbolMap<V>,
}

impl<V> SynSymbolMap<V> {
    pub fn push_inherited(&mut self, inherited_syn_symbol_idx: InheritedSynSymbolIdx, v: V) {
        self.inherited_syn_symbol_map
            .insert_new(inherited_syn_symbol_idx, v)
    }

    pub fn push_current(&mut self, current_syn_symbol_idx: CurrentSynSymbolIdx, v: V) {
        self.current_syn_symbol_map
            .insert_new(current_syn_symbol_idx, v)
    }

    pub fn get_inherited(&self, inherited_syn_symbol_idx: InheritedSynSymbolIdx) -> Option<&V> {
        self.inherited_syn_symbol_map.get(inherited_syn_symbol_idx)
    }

    pub fn get_current(&self, current_syn_symbol_idx: CurrentSynSymbolIdx) -> Option<&V> {
        self.current_syn_symbol_map.get(current_syn_symbol_idx)
    }
}

impl<V> SynSymbolMap<V> {
    pub fn new(syn_symbol_region: &SynSymbolRegion) -> Self {
        Self {
            inherited_syn_symbol_map: InheritedSynSymbolMap::new(
                syn_symbol_region.inherited_syn_symbol_arena(),
            ),
            current_syn_symbol_map: CurrentSynSymbolMap::new(
                syn_symbol_region.current_syn_symbol_arena(),
            ),
        }
    }
}

impl<V> std::ops::Index<InheritedSynSymbolIdx> for SynSymbolMap<V> {
    type Output = V;

    fn index(&self, idx: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_syn_symbol_map[idx]
    }
}

impl<V> std::ops::Index<CurrentSynSymbolIdx> for SynSymbolMap<V> {
    type Output = V;

    fn index(&self, idx: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_syn_symbol_map[idx]
    }
}

pub struct SynSymbolOrderedMap<V> {
    inherited_syn_symbol_ordered_map: InheritedSynSymbolOrderedMap<V>,
    current_syn_symbol_ordered_map: CurrentSynSymbolOrderedMap<V>,
}

impl<V> SynSymbolOrderedMap<V> {
    pub fn push_inherited(&mut self, inherited_syn_symbol_idx: InheritedSynSymbolIdx, v: V) {
        self.inherited_syn_symbol_ordered_map
            .insert_next(inherited_syn_symbol_idx, v)
    }

    pub fn push_current(&mut self, current_syn_symbol_idx: CurrentSynSymbolIdx, v: V) {
        self.current_syn_symbol_ordered_map
            .insert_next(current_syn_symbol_idx, v)
    }
}

impl<V> Default for SynSymbolOrderedMap<V> {
    fn default() -> Self {
        Self {
            inherited_syn_symbol_ordered_map: Default::default(),
            current_syn_symbol_ordered_map: Default::default(),
        }
    }
}

impl<V> std::ops::Index<InheritedSynSymbolIdx> for SynSymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, idx: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_syn_symbol_ordered_map[idx]
    }
}

impl<V> std::ops::Index<CurrentSynSymbolIdx> for SynSymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, idx: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_syn_symbol_ordered_map[idx]
    }
}
