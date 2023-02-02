use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowSelfValue {
    True,
    False,
}

impl AllowSelfValue {
    fn to_bool(self) -> bool {
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
    fn to_bool(self) -> bool {
        match self {
            AllowSelfType::True => true,
            AllowSelfType::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct SymbolRegion {
    inherited_symbol_arena: InheritedSymbolArena,
    current_symbol_arena: CurrentSymbolArena,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
    pattern_ty_constraints: Vec<PatternTypeConstraint>,
}

impl SymbolRegion {
    pub fn regular_parameter_pattern_ty_constraint(
        &self,
        target: PatternExprIdx,
    ) -> Option<ExprIdx> {
        self.pattern_ty_constraints
            .iter()
            .find_map(|pattern_ty_constraint| match pattern_ty_constraint {
                PatternTypeConstraint::RegularParameter { pattern, ty } if *pattern == target => {
                    Some(*ty)
                }
                _ => None,
            })
    }

    pub(crate) fn add_ty_constraint(&mut self, constraint: PatternTypeConstraint) {
        self.pattern_ty_constraints.push(constraint)
    }

    pub fn pattern_ty_constraints(&self) -> &[PatternTypeConstraint] {
        self.pattern_ty_constraints.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternTypeConstraint {
    LetVariables {
        pattern: PatternExprIdx,
        ty: ExprIdx,
    },
    FrameVariable,
    ImplicitTypeParameter,
    RegularParameter {
        pattern: PatternExprIdx,
        ty: ExprIdx,
    },
}

impl SymbolRegion {
    pub(crate) fn new(
        parent_symbol_region: Option<&SymbolRegion>,
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
    pub(crate) fn define_symbols(
        &mut self,
        variables: impl IntoIterator<Item = CurrentSymbol>,
        ty_constraint: Option<PatternTypeConstraint>,
    ) -> ArenaIdxRange<CurrentSymbol> {
        self.pattern_ty_constraints
            .extend(ty_constraint.into_iter());
        self.current_symbol_arena.alloc_batch(variables)
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        self.current_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.token_idx() > token_idx,
                    None => true,
                };
                symbol.ident == ident && accessible
            })
            .map(|(current_symbol_idx, current_symbol)| {
                Symbol::Local(current_symbol_idx, current_symbol.kind())
            })
            .or_else(|| {
                self.inherited_symbol_arena
                    .find_rev_indexed(|symbol| symbol.ident == ident)
                    .map(|(inherited_symbol_idx, inherited_symbol)| {
                        Symbol::Inherited(inherited_symbol_idx, inherited_symbol.kind)
                    })
            })
    }

    pub fn inherited_symbol_iter<'a>(&'a self) -> impl Iterator<Item = &'a InheritedSymbol> + 'a {
        self.inherited_symbol_arena.data().iter()
    }

    pub fn indexed_inherited_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (InheritedSymbolIdx, &'a InheritedSymbol)> + 'a {
        self.inherited_symbol_arena.indexed_iter()
    }

    pub fn indexed_current_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (CurrentSymbolIdx, &'a CurrentSymbol)> + 'a {
        self.current_symbol_arena.indexed_iter()
    }

    pub fn current_symbol_index_iter(&self) -> impl Iterator<Item = CurrentSymbolIdx> {
        self.current_symbol_arena.index_iter()
    }

    fn bequeath(&self) -> InheritedSymbolArena {
        let mut inherited_symbol_arena = InheritedSymbolArena::default();
        for _ in self.indexed_inherited_symbol_iter() {
            todo!()
        }
        for (current_symbol_idx, current_symbol) in self.indexed_current_symbol_iter() {
            let kind = match current_symbol.variant {
                CurrentSymbolVariant::RegularParameter { .. } => {
                    InheritedSymbolKind::RegularParameter
                }
                CurrentSymbolVariant::LetVariable { .. } => todo!(),
                CurrentSymbolVariant::FrameVariable(_) => todo!(),
                CurrentSymbolVariant::ImplicitParameter {
                    ref implicit_parameter_variant,
                } => InheritedSymbolKind::ImplicitParameter,
            };
            inherited_symbol_arena.alloc_one(InheritedSymbol {
                ident: current_symbol.ident,
                kind,
                parent_symbol_idx: current_symbol_idx.into(),
            });
        }
        inherited_symbol_arena
    }

    pub fn allow_self_type(&self) -> AllowSelfType {
        self.allow_self_type
    }

    pub fn allow_self_value(&self) -> AllowSelfValue {
        self.allow_self_value
    }

    pub fn inherited_symbol_arena(&self) -> &InheritedSymbolArena {
        &self.inherited_symbol_arena
    }

    pub fn current_symbol_arena(&self) -> &CurrentSymbolArena {
        &self.current_symbol_arena
    }
}

impl std::ops::Index<InheritedSymbolIdx> for SymbolRegion {
    type Output = InheritedSymbol;

    fn index(&self, index: InheritedSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_arena[index]
    }
}

impl std::ops::Index<CurrentSymbolIdx> for SymbolRegion {
    type Output = CurrentSymbol;

    fn index(&self, index: CurrentSymbolIdx) -> &Self::Output {
        &self.current_symbol_arena[index]
    }
}

pub enum Prevariable {}
