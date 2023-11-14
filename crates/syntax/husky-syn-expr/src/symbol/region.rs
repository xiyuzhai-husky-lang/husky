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
    inherited_syn_symbol_arena: InheritedSynSymbolArena,
    current_syn_symbol_arena: CurrentSynSymbolArena,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    pattern_ty_constraints: Vec<(SyndicateTypeConstraint, CurrentSynSymbolIdxRange)>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyndicateTypeConstraint {
    TemplateTypeParameter,
    OrdinaryParenateParameter {
        syn_pattern_root: ParenateSynPatternExprRoot,
        ty_expr_idx: SynExprIdx,
    },
    VariadicParenateParameter {
        ty: SynExprIdx,
    },
    LetPattern {
        pattern: LetSynPatternExprRoot,
        ty: SynExprIdx,
    },
    FieldVariable {
        ident_token: IdentRegionalToken,
        ty_expr_idx: SynExprIdx,
    },
    LoopVariable,
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
            inherited_syn_symbol_arena: match parent_symbol_region {
                Some(parent_symbol_region) => parent_symbol_region.bequeath(),
                None => Default::default(),
            },
            current_syn_symbol_arena: Default::default(),
            allow_self_type,
            allow_self_value,
            pattern_ty_constraints: vec![],
        }
    }

    #[inline(always)]
    pub(crate) fn define_symbol(
        &mut self,
        variable: CurrentSynSymbol,
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdx {
        let symbol = self.current_syn_symbol_arena.alloc_one(variable);
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
        ty_constraint: Option<SyndicateTypeConstraint>,
    ) -> CurrentSynSymbolIdxRange {
        let symbols = self.current_syn_symbol_arena.alloc_batch(variables);
        self.pattern_ty_constraints.extend(
            ty_constraint
                .into_iter()
                .map(|ty_constraint| (ty_constraint, symbols)),
        );
        symbols
    }

    pub(crate) fn resolve_ident(
        &self,
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
    ) -> Option<Symbol> {
        self.current_syn_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.regional_token_idx() > regional_token_idx,
                    None => true,
                };
                symbol.ident() == Some(ident) && accessible
            })
            .map(|(current_syn_symbol_idx, current_syn_symbol)| {
                Symbol::Current(current_syn_symbol_idx, current_syn_symbol.kind())
            })
            .or_else(|| {
                self.inherited_syn_symbol_arena
                    .find_rev_indexed(|symbol| symbol.ident() == Some(ident))
                    .map(|(inherited_syn_symbol_idx, inherited_syn_symbol)| {
                        Symbol::Inherited(inherited_syn_symbol_idx, inherited_syn_symbol.kind)
                    })
            })
    }

    pub fn inherited_syn_symbols<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a InheritedSynSymbol> + 'a {
        self.inherited_syn_symbol_arena.data().iter()
    }

    pub fn indexed_inherited_syn_symbols<'a>(
        &'a self,
    ) -> impl Iterator<Item = (InheritedSynSymbolIdx, InheritedSynSymbol)> + 'a {
        self.inherited_syn_symbol_arena.indexed_copy_iter()
    }

    pub fn current_syn_symbols<'a>(&'a self) -> impl Iterator<Item = &'a CurrentSynSymbol> + 'a {
        self.current_syn_symbol_arena.iter()
    }

    pub fn indexed_current_syn_symbols<'a>(
        &'a self,
    ) -> impl Iterator<Item = (CurrentSynSymbolIdx, &'a CurrentSynSymbol)> + 'a {
        self.current_syn_symbol_arena.indexed_iter()
    }

    pub fn current_syn_symbol_indices(&self) -> impl Iterator<Item = CurrentSynSymbolIdx> {
        self.current_syn_symbol_arena.indices()
    }

    fn bequeath(&self) -> InheritedSynSymbolArena {
        let mut inherited_syn_symbol_arena = InheritedSynSymbolArena::default();
        for (_, inherited_syn_symbol) in self.indexed_inherited_syn_symbols() {
            inherited_syn_symbol_arena.alloc_one(inherited_syn_symbol);
        }
        for (current_syn_symbol_idx, current_syn_symbol) in self.indexed_current_syn_symbols() {
            let kind = match current_syn_symbol.data {
                CurrentSynSymbolData::ParenateRegularParameter { ident, .. } => {
                    InheritedSynSymbolKind::ParenateParameter { ident }
                }
                CurrentSynSymbolData::LetVariable { .. } => todo!(),
                CurrentSynSymbolData::BeVariable { .. } => todo!(),
                CurrentSynSymbolData::CaseVariable { .. } => todo!(),
                CurrentSynSymbolData::LoopVariable { .. } => todo!(),
                CurrentSynSymbolData::TemplateParameter {
                    ref template_parameter_variant,
                    ..
                } => {
                    InheritedSynSymbolKind::TemplateParameter(template_parameter_variant.bequeath())
                }
                CurrentSynSymbolData::ParenateVariadicParameter { ident_token, .. } => {
                    InheritedSynSymbolKind::ParenateParameter {
                        ident: ident_token.ident(),
                    }
                }
                CurrentSynSymbolData::SelfType => todo!(),
                CurrentSynSymbolData::SelfValue { .. } => todo!(),
                CurrentSynSymbolData::FieldVariable { ident_token } => {
                    InheritedSynSymbolKind::FieldVariable {
                        ident: ident_token.ident(),
                    }
                }
            };
            inherited_syn_symbol_arena.alloc_one(InheritedSynSymbol {
                kind,
                modifier: current_syn_symbol.modifier,
                parent_symbol_idx: current_syn_symbol_idx.into(),
            });
        }
        inherited_syn_symbol_arena
    }

    pub fn allow_self_ty(&self) -> AllowSelfType {
        self.allow_self_type
    }

    pub fn allow_self_value(&self) -> AllowSelfValue {
        self.allow_self_value
    }

    pub fn inherited_syn_symbol_arena(&self) -> &InheritedSynSymbolArena {
        &self.inherited_syn_symbol_arena
    }

    pub fn current_syn_symbol_arena(&self) -> &CurrentSynSymbolArena {
        &self.current_syn_symbol_arena
    }

    // pub fn regular_parameter_pattern_ty_constraint(
    //     &self,
    //     target: SynPatternExprIdx,
    // ) -> Option<SynExprIdx> {
    //     self.pattern_ty_constraints
    //         .iter()
    //         .find_map(|(pattern_ty_constraint, _)| match pattern_ty_constraint {
    //             SyndicateTypeConstraint::ExplicitRegularParameter {
    //                 pattern_expr_idx: pattern,
    //                 ty_expr_idx: ty,
    //             } if *pattern == target => Some(*ty),
    //             _ => None,
    //         })
    // }

    pub fn pattern_ty_constraints(&self) -> &[(SyndicateTypeConstraint, CurrentSynSymbolIdxRange)] {
        &self.pattern_ty_constraints
    }
}

impl std::ops::Index<InheritedSynSymbolIdx> for SynSymbolRegion {
    type Output = InheritedSynSymbol;

    fn index(&self, index: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_syn_symbol_arena[index]
    }
}

impl std::ops::Index<CurrentSynSymbolIdx> for SynSymbolRegion {
    type Output = CurrentSynSymbol;

    fn index(&self, index: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_syn_symbol_arena[index]
    }
}

pub enum Prevariable {}

/// equal to InheritedSynSymbolIdx
///
/// equal to CurrentSynSymbolIdx + number of inherited symbols
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LocalSymbolIdx(usize);

impl From<InheritedSynSymbolIdx> for LocalSymbolIdx {
    fn from(value: InheritedSynSymbolIdx) -> Self {
        Self(value.index())
    }
}

impl LocalSymbolIdx {
    fn from_current_syn_symbol_idx(
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        symbol_region: &SynSymbolRegion,
    ) -> Self {
        Self(symbol_region.inherited_syn_symbol_arena.len() + current_syn_symbol_idx.index())
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
        LocalSymbolIdx::from_current_syn_symbol_idx(self, expr_region_data.symbol_region())
    }
}
