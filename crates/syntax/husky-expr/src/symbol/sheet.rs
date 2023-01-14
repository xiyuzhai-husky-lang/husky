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
pub struct SymbolSheet {
    inherited_symbol_arena: InheritedSymbolArena,
    local_symbol_arena: LocalSymbolArena,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
}

impl SymbolSheet {
    pub(crate) fn new(
        parent_symbol_sheet: Option<&SymbolSheet>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        #[cfg(test)]
        {
            if allow_self_value.to_bool() {
                assert!(allow_self_type.to_bool());
            } else {
                if let Some(parent_symbol_sheet) = parent_symbol_sheet {
                    assert!(!parent_symbol_sheet.allow_self_value.to_bool());
                }
            }
            if !allow_self_type.to_bool() {
                if let Some(parent_symbol_sheet) = parent_symbol_sheet {
                    assert!(!parent_symbol_sheet.allow_self_type.to_bool());
                }
            }
        }
        Self {
            // ad hoc
            inherited_symbol_arena: match parent_symbol_sheet {
                Some(parent_symbol_sheet) => parent_symbol_sheet.bequeath(),
                None => Default::default(),
            },
            local_symbol_arena: Default::default(),
            allow_self_type,
            allow_self_value,
        }
    }

    #[inline(always)]
    pub(crate) fn define_variables(
        &mut self,
        variables: Vec<LocalSymbol>,
    ) -> ArenaIdxRange<LocalSymbol> {
        self.local_symbol_arena.alloc_batch(variables)
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        self.local_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.token_idx() > token_idx,
                    None => true,
                };
                symbol.ident == ident && accessible
            })
            .map(|(local_symbol_idx, local_symbol)| {
                Symbol::Local(local_symbol_idx, local_symbol.kind)
            })
            .or_else(|| {
                self.inherited_symbol_arena
                    .find_rev_indexed(|symbol| symbol.ident == ident)
                    .map(|(inherited_symbol_idx, inherited_symbol)| {
                        Symbol::Inherited(inherited_symbol_idx, inherited_symbol.kind)
                    })
            })
    }

    pub fn indexed_inherited_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (InheritedSymbolIdx, &'a InheritedSymbol)> + 'a {
        self.inherited_symbol_arena.indexed_iter()
    }

    pub fn indexed_local_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (LocalSymbolIdx, &'a LocalSymbol)> + 'a {
        self.local_symbol_arena.indexed_iter()
    }

    fn bequeath(&self) -> InheritedSymbolArena {
        let mut inherited_symbol_arena = InheritedSymbolArena::default();
        for _ in self.indexed_inherited_symbol_iter() {
            todo!()
        }
        for (original_local_symbol_idx, local_symbol) in self.indexed_local_symbol_iter() {
            inherited_symbol_arena.alloc_one(match local_symbol.kind {
                LocalSymbolKind::Parameter { .. } => InheritedSymbol {
                    ident: local_symbol.ident,
                    kind: InheritedSymbolKind::Parameter {
                        original_local_symbol_idx,
                    },
                },
                LocalSymbolKind::LetVariable { .. } => todo!(),
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
}

impl std::ops::Index<InheritedSymbolIdx> for SymbolSheet {
    type Output = InheritedSymbol;

    fn index(&self, index: InheritedSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_arena[index]
    }
}

impl std::ops::Index<LocalSymbolIdx> for SymbolSheet {
    type Output = LocalSymbol;

    fn index(&self, index: LocalSymbolIdx) -> &Self::Output {
        &self.local_symbol_arena[index]
    }
}

pub enum Prevariable {}
