use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolSheet {
    inherited_symbol_arena: InheritedSymbolArena,
    local_symbol_arena: LocalSymbolArena,
}

impl SymbolSheet {
    pub(crate) fn new() -> Self {
        Self {
            // ad hoc
            inherited_symbol_arena: Default::default(),
            local_symbol_arena: Default::default(),
        }
    }

    #[inline(always)]
    pub(crate) fn define_variables(
        &mut self,
        variables: Vec<LocalSymbol>,
    ) -> ArenaIdxRange<LocalSymbol> {
        self.local_symbol_arena.alloc_batch(variables)
    }

    pub(crate) fn resolve_ident(
        &self,
        token_idx: TokenIdx,
        ident: Identifier,
    ) -> Option<(LocalSymbolIdx, LocalSymbolKind)> {
        // ad hoc
        self.local_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.token_idx() > token_idx,
                    None => todo!(),
                };
                symbol.ident == ident && accessible
            })
            .map(|(local_symbol_idx, local_symbol)| (local_symbol_idx, local_symbol.kind))
    }

    pub fn indexed_local_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (LocalSymbolIdx, &'a LocalSymbol)> + 'a {
        self.local_symbol_arena.indexed_iter()
    }
}

impl std::ops::Index<LocalSymbolIdx> for SymbolSheet {
    type Output = LocalSymbol;

    fn index(&self, index: LocalSymbolIdx) -> &Self::Output {
        &self.local_symbol_arena[index]
    }
}

pub enum Prevariable {}
