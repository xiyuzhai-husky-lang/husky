use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolMap<V> {
    inherited_symbol_map: InheritedSymbolMap<V>,
    current_symbol_map: CurrentSymbolMap<V>,
}

impl<V> SymbolMap<V> {
    pub fn new(parent: Option<&Self>, region: &SymbolRegion) -> Self
    where
        V: Clone,
    {
        let inherited_symbol_arena = region.inherited_symbol_arena();
        let current_symbol_arena = region.current_symbol_arena();
        Self {
            inherited_symbol_map: match parent {
                Some(parent) => {
                    let mut inherited_symbol_map = parent
                        .inherited_symbol_map
                        .clone_for_extended(inherited_symbol_arena);
                    let base = parent.inherited_symbol_map.len();
                    for (parent_idx, v) in parent.current_symbol_map.key_value_iter() {
                        unsafe {
                            let idx = ArenaIdx::from_raw(base + parent_idx.raw());
                            inherited_symbol_map.insert_new(idx, v.clone())
                        }
                    }
                    inherited_symbol_map
                }
                None => InheritedSymbolMap::new(inherited_symbol_arena),
            },
            current_symbol_map: CurrentSymbolMap::new(current_symbol_arena),
        }
    }

    pub fn insert_new(&mut self, idx: CurrentSymbolIdx, v: V) {
        self.current_symbol_map.insert_new(idx, v)
    }

    pub fn inherited_symbol_map(&self) -> &InheritedSymbolMap<V> {
        &self.inherited_symbol_map
    }

    pub fn current_symbol_map(&self) -> &CurrentSymbolMap<V> {
        &self.current_symbol_map
    }
}

impl<V> std::ops::Index<InheritedSymbolIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: InheritedSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_map[index]
    }
}

impl<V> std::ops::Index<CurrentSymbolIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: CurrentSymbolIdx) -> &Self::Output {
        &self.current_symbol_map[index]
    }
}
