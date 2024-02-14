mod context_mut;
mod map;
mod ordered_map;
mod region;

pub use self::context_mut::*;
pub use self::map::*;
pub use self::ordered_map::*;
pub use self::region::*;

use crate::*;
use husky_entity_tree::ModuleSymbolContext;
use husky_term_prelude::symbol::SymbolName;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::{IsStreamParser, TryParseFromStream};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(InheritedSynSymbolIdx, InheritedSynSymbolKind),
    Current(CurrentSynSymbolIdx, CurrentSynSymbolKind),
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitParameterSymbol {
    Lifetime {
        label_token: LifetimeLabelRegionalToken,
    },
    Type {
        ident_token: IdentRegionalToken,
    },
    Const {},
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InheritedSynSymbol {
    parent_symbol_idx: ParentSynSymbolIdx,
    modifier: SvarModifier,
    kind: InheritedSynSymbolKind,
}

impl InheritedSynSymbol {
    pub fn kind(&self) -> InheritedSynSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSynSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn name(&self) -> SymbolName {
        match self.kind {
            InheritedSynSymbolKind::TemplateParameter(
                InheritedTemplateParameterSynSymbol::Lifetime { label, .. }
                | InheritedTemplateParameterSynSymbol::Place { label, .. },
            ) => label.into(),
            InheritedSynSymbolKind::TemplateParameter(
                InheritedTemplateParameterSynSymbol::Type { ident }
                | InheritedTemplateParameterSynSymbol::Constant { ident },
            )
            | InheritedSynSymbolKind::ParenateParameter { ident }
            | InheritedSynSymbolKind::FieldVariable { ident } => ident.into(),
        }
    }

    pub fn ident(&self) -> Option<Ident> {
        self.name().ident()
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedSynSymbolKind {
    TemplateParameter(InheritedTemplateParameterSynSymbol),
    ParenateParameter { ident: Ident },
    FieldVariable { ident: Ident },
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedTemplateParameterSynSymbol {
    Lifetime { label: Label },
    Place { label: Label },
    Type { ident: Ident },
    Constant { ident: Ident },
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct CurrentSynSymbol {
    modifier: SvarModifier,
    access_start: RegionalTokenIdx,
    /// this is none for template, parenate, lambda and field variable
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
}

/// # getters
impl CurrentSynSymbol {
    pub fn modifier(&self) -> SvarModifier {
        self.modifier
    }

    pub fn kind(&self) -> CurrentSynSymbolKind {
        self.data.kind()
    }

    pub fn data(&self) -> &CurrentSynSymbolData {
        &self.data
    }

    pub fn name(&self) -> SymbolName {
        match self.data {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | CurrentSynSymbolData::VariadicParenateParameter { ident_token, .. }
            | CurrentSynSymbolData::FieldVariable { ident_token } => ident_token.ident().into(),
            CurrentSynSymbolData::SimpleParenateParameter { ident, .. }
            | CurrentSynSymbolData::LetVariable { ident, .. }
            | CurrentSynSymbolData::BeVariable { ident, .. }
            | CurrentSynSymbolData::CaseVariable { ident, .. }
            | CurrentSynSymbolData::LoopVariable { ident, .. }
            | CurrentSynSymbolData::SimpleLambdaParameter { ident, .. } => ident.into(),
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token, .. },
                ..
            } => label_token.label().into(),
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Place { label_token, .. },
                ..
            } => label_token.label().into(),
            CurrentSynSymbolData::SelfType => SymbolName::SelfType,
            CurrentSynSymbolData::SelfValue { .. } => SymbolName::SelfValue,
        }
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.data {
            CurrentSynSymbolData::TemplateParameter {
                template_parameter_variant:
                    CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. }
                    | CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. },
                ..
            }
            | CurrentSynSymbolData::VariadicParenateParameter { ident_token, .. }
            | CurrentSynSymbolData::FieldVariable { ident_token } => Some(ident_token.ident()),
            CurrentSynSymbolData::SimpleParenateParameter { ident, .. }
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
            CurrentSynSymbolData::SimpleLambdaParameter {
                ident,
                pattern_symbol_idx,
            } => todo!(),
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentSynSymbolKind {
    TemplateParameter {
        template_parameter_kind: CurrentTemplateParameterSynSymbolKind,
    },
    ParenateSimpleParameter {
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    ParenateVariadicParameter {
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

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentTemplateParameterSynSymbolKind {
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

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
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
    SimpleParenateParameter {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
    },
    VariadicParenateParameter {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
    },
    SimpleLambdaParameter {
        ident: Ident,
        pattern_symbol_idx: SynPatternSymbolIdx,
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
            if let Some(attr_token) = sp.try_parse_option::<AttrRegionalToken>()? {
                let syn_attr = match attr_token {
                    AttrRegionalToken::Phantom(phantom_token) => {
                        TemplateSymbolSynAttr::Phantom(pound, phantom_token)
                    }
                    AttrRegionalToken::Runtime(runtime_token) => {
                        TemplateSymbolSynAttr::Runtime(pound, runtime_token)
                    }
                };
                syn_attrs.push(syn_attr)
            };
        }
        Ok(TemplateParameterSynAttrs { syn_attrs })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TemplateSymbolSynAttr {
    Phantom(PoundRegionalToken, PhantomRegionalToken),
    Runtime(PoundRegionalToken, RuntimeRegionalToken),
}

impl CurrentSynSymbolData {
    fn symbol_modifier(&self, pattern_expr_region: &SynPatternExprRegion) -> SvarModifier {
        match self {
            CurrentSynSymbolData::TemplateParameter { .. } => SvarModifier::Const,
            CurrentSynSymbolData::SimpleParenateParameter {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolData::SimpleLambdaParameter {
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
            CurrentSynSymbolData::VariadicParenateParameter {
                symbol_modifier_keyword_group,
                ..
            } => SvarModifier::new(*symbol_modifier_keyword_group),
            CurrentSynSymbolData::LoopVariable { .. } => SvarModifier::Pure,
            CurrentSynSymbolData::SelfType => SvarModifier::Const,
            CurrentSynSymbolData::SelfValue {
                symbol_modifier_keyword_group,
            } => SvarModifier::new(*symbol_modifier_keyword_group),
            CurrentSynSymbolData::FieldVariable { .. } => SvarModifier::Pure,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
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
            CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. } => {
                InheritedTemplateParameterSynSymbol::Constant {
                    ident: ident_token.ident(),
                }
            }
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
            CurrentSynSymbolData::SimpleParenateParameter {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::ParenateSimpleParameter {
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
            CurrentSynSymbolData::VariadicParenateParameter { ident_token, .. } => {
                CurrentSynSymbolKind::ParenateVariadicParameter {
                    ident_token: *ident_token,
                }
            }
            CurrentSynSymbolData::SelfType => todo!(),
            CurrentSynSymbolData::SelfValue { .. } => todo!(),
            CurrentSynSymbolData::FieldVariable { ident_token } => {
                CurrentSynSymbolKind::FieldVariable {
                    ident_token: *ident_token,
                }
            }
            CurrentSynSymbolData::SimpleLambdaParameter {
                ident,
                pattern_symbol_idx,
            } => todo!(),
        }
    }
}

impl CurrentTemplateParameterSynSymbolVariant {
    fn kind(&self) -> CurrentTemplateParameterSynSymbolKind {
        match self {
            CurrentTemplateParameterSynSymbolVariant::Type { ident_token, .. } => {
                CurrentTemplateParameterSynSymbolKind::Type {
                    ident_token: *ident_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Lifetime { label_token } => {
                CurrentTemplateParameterSynSymbolKind::Lifetime {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Place { label_token } => {
                CurrentTemplateParameterSynSymbolKind::Place {
                    label_token: *label_token,
                }
            }
            CurrentTemplateParameterSynSymbolVariant::Constant { ident_token, .. } => {
                CurrentTemplateParameterSynSymbolKind::Constant {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

pub type InheritedVariableArena = Arena<InheritedSynSymbol>;
pub type InheritedSynSymbolIdx = ArenaIdx<InheritedSynSymbol>;
pub type InheritedSynSymbolIdxRange = ArenaIdxRange<InheritedSynSymbol>;
pub(crate) type InheritedSynSymbolMap<V> = ArenaMap<InheritedSynSymbol, V>;
pub(crate) type InheritedSynSymbolOrderedMap<V> = ArenaOrderedMap<InheritedSynSymbol, V>;

pub type CurrentVariableArena = Arena<CurrentSynSymbol>;
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
    pub fn new(syn_symbol_region: &VariableRegionData) -> Self {
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
