use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowSelfValue {
    True,
    False,
}

impl AllowSelfValue {
    pub fn to_bool(self) -> bool {
        match self {
            AllowSelfValue::True => true,
            AllowSelfValue::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowSelfType {
    True,
    False,
}

impl AllowSelfType {
    pub fn to_bool(self) -> bool {
        match self {
            AllowSelfType::True => true,
            AllowSelfType::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynSymbolRegion {
    inherited_symbol_arena: InheritedSynSymbolArena,
    current_symbol_arena: CurrentSynSymbolArena,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    pattern_ty_constraints: Vec<(PatternTypeConstraint, CurrentSynSymbolIdxRange)>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternTypeConstraint {
    TemplateTypeParameter,
    ExplicitRegularParameter {
        pattern_expr_idx: SynPatternExprIdx,
        ty_expr_idx: SynExprIdx,
    },
    ExplicitVariadicParameter {
        ty: SynExprIdx,
    },
    LetVariables {
        pattern: SynPatternExprIdx,
        ty: SynExprIdx,
    },
    FrameVariable,
}

impl SynSymbolRegion {
    pub(crate) fn new(
        parent_symbol_region: Option<&SynSymbolRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        #[cfg(test)]
        {
            if allow_self_value.to_bool() {
                assert!(allow_self_type.to_bool());
            } else {
                if let Some(parent_symbol_region) = parent_symbol_region {
                    assert!(!parent_symbol_region.allow_self_value.to_bool());
                }
            }
            if !allow_self_type.to_bool() {
                if let Some(parent_symbol_region) = parent_symbol_region {
                    assert!(!parent_symbol_region.allow_self_type.to_bool());
                }
            }
        }
        Self {
            // ad hoc
            inherited_symbol_arena: match parent_symbol_region {
                Some(parent_symbol_region) => parent_symbol_region.bequeath(),
                None => Default::default(),
            },
            current_symbol_arena: Default::default(),
            allow_self_type,
            allow_self_value,
            pattern_ty_constraints: vec![],
        }
    }

    #[inline(always)]
    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSynSymbol,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSynSymbolIdx {
        let symbol = self.current_symbol_arena.alloc_one(variable);
        self.pattern_ty_constraints.extend(
            ty_constraint
                .into_iter()
                .map(|ty_constraint| (ty_constraint, CurrentSynSymbolIdxRange::new_single(symbol))),
        );
        symbol
    }

    #[inline(always)]
    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSynSymbol>,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> CurrentSynSymbolIdxRange {
        let symbols = self.current_symbol_arena.alloc_batch(variables);
        self.pattern_ty_constraints.extend(
            ty_constraint
                .into_iter()
                .map(|ty_constraint| (ty_constraint, symbols)),
        );
        symbols
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Ident) -> Option<Symbol> {
        self.current_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.token_idx() > token_idx,
                    None => true,
                };
                symbol.ident() == Some(ident) && accessible
            })
            .map(|(current_symbol_idx, current_symbol)| {
                Symbol::Local(current_symbol_idx, current_symbol.kind())
            })
            .or_else(|| {
                self.inherited_symbol_arena
                    .find_rev_indexed(|symbol| symbol.ident() == Some(ident))
                    .map(|(inherited_symbol_idx, inherited_symbol)| {
                        Symbol::Inherited(inherited_symbol_idx, inherited_symbol.kind)
                    })
            })
    }

    pub fn inherited_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a InheritedSynSymbol> + 'a {
        self.inherited_symbol_arena.data().iter()
    }

    pub fn indexed_inherited_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (InheritedSynSymbolIdx, InheritedSynSymbol)> + 'a {
        self.inherited_symbol_arena.indexed_copy_iter()
    }

    pub fn current_symbol_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (CurrentSynSymbolIdx, &'a CurrentSynSymbol)> + 'a {
        self.current_symbol_arena.indexed_iter()
    }

    pub fn current_symbol_index_iter(&self) -> impl Iterator<Item = CurrentSynSymbolIdx> {
        self.current_symbol_arena.index_iter()
    }

    fn bequeath(&self) -> InheritedSynSymbolArena {
        let mut inherited_symbol_arena = InheritedSynSymbolArena::default();
        for (_, inherited_symbol) in self.indexed_inherited_symbol_iter() {
            inherited_symbol_arena.alloc_one(inherited_symbol);
        }
        for (current_symbol_idx, current_symbol) in self.current_symbol_indexed_iter() {
            let kind = match current_symbol.variant {
                CurrentSynSymbolVariant::ParenateRegularParameter { ident, .. } => {
                    InheritedSynSymbolKind::ExplicitParameter { ident }
                }
                CurrentSynSymbolVariant::LetVariable { .. } => todo!(),
                CurrentSynSymbolVariant::FrameVariable { .. } => todo!(),
                CurrentSynSymbolVariant::TemplateParameter {
                    ref template_parameter_variant,
                    ..
                } => {
                    InheritedSynSymbolKind::ImplicitParameter(template_parameter_variant.bequeath())
                }
                CurrentSynSymbolVariant::ParenateVariadicParameter { ident_token, .. } => {
                    InheritedSynSymbolKind::ExplicitParameter {
                        ident: ident_token.ident(),
                    }
                }
                CurrentSynSymbolVariant::SelfType => todo!(),
                CurrentSynSymbolVariant::SelfValue {
                    symbol_modifier_keyword_group,
                } => todo!(),
            };
            inherited_symbol_arena.alloc_one(InheritedSynSymbol {
                kind,
                modifier: current_symbol.modifier,
                parent_symbol_idx: current_symbol_idx.into(),
            });
        }
        inherited_symbol_arena
    }

    pub fn allow_self_ty(&self) -> AllowSelfType {
        self.allow_self_type
    }

    pub fn allow_self_value(&self) -> AllowSelfValue {
        self.allow_self_value
    }

    pub fn inherited_symbol_arena(&self) -> &InheritedSynSymbolArena {
        &self.inherited_symbol_arena
    }

    pub fn current_symbol_arena(&self) -> &CurrentSynSymbolArena {
        &self.current_symbol_arena
    }

    pub fn regular_parameter_pattern_ty_constraint(
        &self,
        target: SynPatternExprIdx,
    ) -> Option<SynExprIdx> {
        self.pattern_ty_constraints
            .iter()
            .find_map(|(pattern_ty_constraint, _)| match pattern_ty_constraint {
                PatternTypeConstraint::ExplicitRegularParameter {
                    pattern_expr_idx: pattern,
                    ty_expr_idx: ty,
                } if *pattern == target => Some(*ty),
                _ => None,
            })
    }

    pub fn pattern_ty_constraints(&self) -> &[(PatternTypeConstraint, CurrentSynSymbolIdxRange)] {
        &self.pattern_ty_constraints
    }
}

impl std::ops::Index<InheritedSynSymbolIdx> for SynSymbolRegion {
    type Output = InheritedSynSymbol;

    fn index(&self, index: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_arena[index]
    }
}

impl std::ops::Index<CurrentSynSymbolIdx> for SynSymbolRegion {
    type Output = CurrentSynSymbol;

    fn index(&self, index: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_symbol_arena[index]
    }
}

pub enum Prevariable {}

/// equal to InheritedSymbolIdx
///
/// equal to CurrentSymbolIdx + number of inherited symbols
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LocalSymbolIdx(usize);

impl From<InheritedSynSymbolIdx> for LocalSymbolIdx {
    fn from(value: InheritedSynSymbolIdx) -> Self {
        Self(value.raw())
    }
}

impl LocalSymbolIdx {
    fn from_current_symbol_idx(
        current_symbol_idx: CurrentSynSymbolIdx,
        symbol_region: &SynSymbolRegion,
    ) -> Self {
        Self(symbol_region.inherited_symbol_arena.len() + current_symbol_idx.raw())
    }
}

pub trait IntoLocalSymbolIdx: Copy {
    fn into_local_symbol_idx(self, expr_region_data: &SynExprRegionData) -> LocalSymbolIdx;
}

impl IntoLocalSymbolIdx for InheritedSynSymbolIdx {
    fn into_local_symbol_idx(self, _: &SynExprRegionData) -> LocalSymbolIdx {
        self.into()
    }
}

impl IntoLocalSymbolIdx for CurrentSynSymbolIdx {
    fn into_local_symbol_idx(self, expr_region_data: &SynExprRegionData) -> LocalSymbolIdx {
        LocalSymbolIdx::from_current_symbol_idx(self, expr_region_data.symbol_region())
    }
}
